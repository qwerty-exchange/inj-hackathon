#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, ensure_ne, to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo,
    Order, Response, StdResult, Storage,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;
use cw_utils::NativeBalance;
use crate::ensure_state;
use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, PropositionState, PropositionType, QueryMsg, SudoMsg, MigrateMsg,
};
use crate::state::{Proposition, PROPOSITIONS, PROPOSITION_COUNT};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:qwerty:crypto-pawn";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateProposition {
            proposition_type,
            deposit,
            assets,
            premium,
            period,
            expiry,
            contractor,
        } => create_proposition(
            deps,
            env,
            _info,
            proposition_type,
            deposit,
            assets,
            premium,
            period,
            expiry,
            contractor,
        ),

        ExecuteMsg::AcceptProposition { proposition_id } => {
            accept_proposition(deps, env, _info, proposition_id)
        }

        ExecuteMsg::RejectProposition { proposition_id } => {
            reject_proposition(deps, env, _info, proposition_id)
        }

        ExecuteMsg::CloseProposition { proposition_id } => {
            close_proposition(deps, env, _info, proposition_id)
        }
    }
}

#[entry_point]
pub fn sudo(_deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::BeginBlocker {} => Ok(Response::new()),
    }
}

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPropositions {
            start_before,
            limit,
        } => {
            let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
            let end = start_before.map(Bound::exclusive);
            let props: StdResult<Vec<_>> = PROPOSITIONS
                .range(deps.storage, None, end, Order::Descending)
                .take(limit)
                .collect();

            to_binary(&props.unwrap_or_default())
        }
        QueryMsg::GetPropositionCount {} => {
            let proposition_count = PROPOSITION_COUNT
                .may_load(deps.storage)?
                .unwrap_or_default();
            to_binary(&format!("{proposition_count}"))
        }
        QueryMsg::GetProposition { proposition_id } => {
            let proposition = PROPOSITIONS.load(deps.storage, proposition_id)?;
            to_binary(&proposition)
        }
    }
}

pub fn create_proposition(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    proposition_type: PropositionType,
    deposit: Coin,
    assets: Coin,
    premium: Coin,
    period: u64,
    expiry: u64,
    contractor: Option<Addr>,
) -> Result<Response, ContractError> {
    // let funds = &info.funds.clone();
    let mut balance = NativeBalance(info.funds.clone());
    if proposition_type == PropositionType::Ask {
        balance = (balance - deposit.to_owned())?;
        balance = (balance - premium.to_owned())?;
    }

    if proposition_type == PropositionType::Bid {
        (balance - assets.to_owned())?;
    }

    let proposition = Proposition {
        owner: info.sender,
        proposition_type,
        state: PropositionState::Active,
        deposit,
        assets,
        premium,
        period,
        expiry,
        contractor,
    };

    let id = next_id(deps.storage)?;
    PROPOSITIONS.save(deps.storage, id, &proposition)?;

    Ok(
        Response::new()
        .add_attribute("method", "create_proposition")
        .add_attribute("proposition_id", id.to_string())
    )
}

pub fn accept_proposition(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    proposition_id: u64,
) -> Result<Response, ContractError> {
    let mut proposition = PROPOSITIONS.load(deps.storage, proposition_id)?;

    ensure_ne!(
        proposition.owner,
        info.sender,
        ContractError::Unauthorized {} // todo: Another error
    );

    ensure_state!(proposition.state, PropositionState::Active);

    ensure!(
        proposition.is_not_expired(&env),
        ContractError::Std(cosmwasm_std::StdError::GenericErr {
            msg: "Proposition is expired".to_owned()
        })
    );

    if proposition.contractor.is_some() && proposition.contractor.unwrap() == info.sender {
        return Err(ContractError::Unauthorized {}); // todo: Another error
    }
    proposition.contractor = Some(info.sender);

    proposition.expiry = env.block.time.plus_seconds(proposition.period).seconds();
    proposition.state = PropositionState::Accepted;

    PROPOSITIONS.save(deps.storage, proposition_id, &proposition)?;

    let mut msgs: Vec<BankMsg> = Vec::new();
    let mut balance = NativeBalance(info.funds.clone());
    if proposition.proposition_type == PropositionType::Ask {
        (balance - proposition.assets.clone())?;
    } else {
        balance = (balance - proposition.deposit.clone())?;
        (balance - proposition.premium.clone())?;
    }

    msgs.push(
        send_coins_from_contract_to_message(
            &proposition.get_lender(),
            [proposition.assets.clone()].to_vec(),
        )
        .unwrap(),
    );

    msgs.push(
        send_coins_from_contract_to_message(
            &proposition.get_borrower(),
            [proposition.premium.clone()].to_vec(),
        )
        .unwrap(),
    );

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("method", "accept_proposition")
        .add_attribute("id", proposition_id.to_string())
        .add_attribute("proposition_type", proposition.proposition_type.to_string())
        .add_attribute("owner", proposition.owner)
        .add_attribute("contractor", proposition.contractor.unwrap())
    )
        
}

pub fn reject_proposition(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    proposition_id: u64,
) -> Result<Response, ContractError> {
    let mut proposition = PROPOSITIONS.load(deps.storage, proposition_id)?;

    if proposition.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    ensure_state!(proposition.state, PropositionState::Active);

    proposition.state = PropositionState::Rejected;

    PROPOSITIONS.save(deps.storage, proposition_id, &proposition)?;

    let return_coins: Vec<Coin> = match proposition.proposition_type {
        PropositionType::Ask => [proposition.deposit.clone(), proposition.premium.clone()].to_vec(),
        PropositionType::Bid => [proposition.assets.clone()].to_vec(),
    };

    Ok(Response::new()
        .add_message(send_coins_from_contract_to_message(
            &proposition.owner,
            return_coins,
        )?)
        .add_attribute("method", "reject_proposition"))
}

pub fn close_proposition(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    proposition_id: u64,
) -> Result<Response, ContractError> {
    let mut proposition = PROPOSITIONS.load(deps.storage, proposition_id)?;

    ensure_state!(proposition.state, PropositionState::Accepted);

    proposition.state = PropositionState::Closed;

    let mut bank_msgs: Vec<BankMsg> = Vec::new();

    if proposition.is_not_expired(&env) {
        ensure!(
            proposition.get_lender() == info.sender,
            ContractError::Unauthorized {}
        );

        (NativeBalance(info.funds.clone()) - proposition.assets.clone())?;

        bank_msgs.push(
            send_coins_from_contract_to_message(
                &proposition.get_borrower(),
                [proposition.assets.clone()].to_vec(),
            )
            .unwrap(),
        );

        bank_msgs.push(
            send_coins_from_contract_to_message(
                &proposition.get_lender(),
                [proposition.deposit.clone()].to_vec(),
            )
            .unwrap(),
        );
    } else {
        ensure!(
            proposition.get_borrower() == info.sender,
            ContractError::Unauthorized {}
        );

        bank_msgs.push(
            send_coins_from_contract_to_message(
                &proposition.get_borrower(),
                [proposition.deposit.clone()].to_vec(),
            )
            .unwrap(),
        );
    }

    PROPOSITIONS.save(deps.storage, proposition_id, &proposition)?;

    Ok(Response::new()
        .add_messages(bank_msgs)
        .add_attribute("method", "close_proposition")
        .add_attribute("id", proposition_id.to_string())
        .add_attribute("proposition_type", proposition.proposition_type.to_string())
        .add_attribute("owner", proposition.owner)
        .add_attribute("contractor", proposition.contractor.unwrap())
    )
}

pub fn next_id(store: &mut dyn Storage) -> StdResult<u64> {
    let id: u64 = PROPOSITION_COUNT.may_load(store)?.unwrap_or_default() + 1;
    PROPOSITION_COUNT.save(store, &id)?;
    Ok(id)
}

pub fn send_coins_from_contract_to_message(
    to_address: &Addr,
    coins: Vec<Coin>,
) -> StdResult<BankMsg> {
    let mut balance = NativeBalance(coins);
    balance.normalize();

    Ok(BankMsg::Send {
        to_address: to_address.to_string(),
        amount: balance.into_vec(),
    })
}

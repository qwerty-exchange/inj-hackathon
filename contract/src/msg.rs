use std::fmt;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};


#[cw_serde]
pub struct InstantiateMsg {}
#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateProposition {
        proposition_type: PropositionType,
        deposit: Coin,
        assets: Coin,
        premium: Coin,
        period: u64,
        expiry: u64,
        contractor: Option<Addr>,
    },
    RejectProposition {
        proposition_id: u64,
    },
    AcceptProposition {
        proposition_id: u64,
    },
    CloseProposition {
        proposition_id: u64,
    },
}

#[cw_serde]
pub enum PropositionType {
    Ask,
    Bid,
}
impl fmt::Display for PropositionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[cw_serde]
pub enum PropositionState {
    Active,
    Accepted,
    Closed,
    Rejected,
}


#[cw_serde]
pub enum SudoMsg {
    BeginBlocker {},
}

#[cw_serde]
// #[derive(QueryResponses)]
pub enum QueryMsg {
    // #[returns(Vec<Proposition>)]
    GetPropositions {
        start_before: Option<u64>,
        limit: Option<u32>,
    },
    // #[returns(Proposition)]
    GetProposition {
        proposition_id: u64,
    },
    // #[returns(Uint64)]
    GetPropositionCount {},
}

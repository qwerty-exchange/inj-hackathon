use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Env};
use cw_storage_plus::{Item, Map};

use crate::msg::{PropositionState, PropositionType};

#[cw_serde]
pub struct Proposition {
    pub owner: Addr,
    pub proposition_type: PropositionType,
    pub state: PropositionState,
    pub deposit: Coin,
    pub assets: Coin,
    pub premium: Coin,
    pub period: u64,
    pub expiry: u64,
    pub contractor: Option<Addr>,
}

impl Proposition {
    pub fn get_lender(&self) -> Addr {
        match self.proposition_type {
            PropositionType::Ask {} => self.owner.clone(),
            PropositionType::Bid {} => self.contractor.clone().unwrap(),
        }
    }

    pub fn get_borrower(&self) -> Addr {
        match self.proposition_type {
            PropositionType::Ask {} => self.contractor.clone().unwrap(),
            PropositionType::Bid {} => self.owner.clone(),
        }
    }

    pub fn is_expired(&self, env: &Env) -> bool {
        env.block.time.seconds() > self.expiry
    }

    pub fn is_not_expired(&self, env: &Env) -> bool {
        !self.is_expired(env)
    }
}

pub const PROPOSITIONS: Map<u64, Proposition> = Map::new("propositions");
pub const PROPOSITION_COUNT: Item<u64> = Item::new("proposition_count");

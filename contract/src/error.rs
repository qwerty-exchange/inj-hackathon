use cosmwasm_std::{Coin, StdError};
use thiserror::Error;

use crate::msg::PropositionState;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Unauthorized")]
    Unauthorized {},
    #[error("PaymentError")]
    PaymentError {
        expected: Coin,
        actual: Option<Coin>,
    },
    #[error("WrongPropositionStatus")]
    WrongPropositionStatus {
        expected: PropositionState,
        current: PropositionState,
    },
}

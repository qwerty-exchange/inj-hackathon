use cosmwasm_schema::cw_serde;

use crate::msg::ExecuteMsg;
use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, StdResult, WasmMsg };

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[cw_serde]
pub struct CwTemplateContract(pub Addr);


impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    //   pub fn query<Q, T, CQ>(&self, querier: &Q, msg: T) -> StdResult<CosmosMsg> {
    //     let msg = to_binary(&msg.into())?;
    //     Ok(WasmQuery::Smart {
    //       contract_addr: self.addr().into(),
    //         msg,
    //     }
    //     .into());

    //     let r =  QuerierWrapper::<CQ>::new(querier).query(&query)?;

    //     Ok(())
    // }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    pub fn call_with_funds<T: Into<ExecuteMsg>>(
        &self,
        msg: T,
        funds: Vec<Coin>,
    ) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds,
        }
        .into())
    }
}

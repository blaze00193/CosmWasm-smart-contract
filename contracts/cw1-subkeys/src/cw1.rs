use cosmwasm_std::{ensure, Addr, DepsMut, Env, MessageInfo, Response, StdResult};
use cw1::Cw1;

use crate::contract::Cw1SubkeysContract;
use crate::error::ContractError;

impl Cw1 for Cw1SubkeysContract<'_> {
    type Error = ContractError;

    fn execute(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        msgs: Vec<cosmwasm_std::CosmosMsg>,
    ) -> Result<cosmwasm_std::Response, Self::Error> {
        let (deps, env, info) = ctx;
        let authorized: StdResult<_> = msgs.iter().fold(Ok(true), |acc, msg| {
            Ok(acc? & self.is_authorized(deps.as_ref(), &env, &info.sender, msg)?)
        });

        ensure!(authorized?, ContractError::Unauthorized {});

        let res = Response::new()
            .add_messages(msgs)
            .add_attribute("action", "execute")
            .add_attribute("owner", info.sender);
        Ok(res)
    }

    fn can_execute(
        &self,
        ctx: (cosmwasm_std::Deps, cosmwasm_std::Env),
        sender: String,
        msg: cosmwasm_std::CosmosMsg,
    ) -> StdResult<cw1::CanExecuteResp> {
        let (deps, env) = ctx;
        let sender = Addr::unchecked(&sender);

        self.is_authorized(deps, &env, &sender, &msg)
            .map(|can| cw1::CanExecuteResp { can_execute: can })
    }
}
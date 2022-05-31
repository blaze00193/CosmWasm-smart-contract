#![allow(dead_code)]
use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo, Response};

use sylvia_derive::interface;

#[interface(module=msg)]
pub trait Cw1<Msg>
where
    Msg: std::fmt::Debug + PartialEq + Clone + schemars::JsonSchema,
{
    type Error;

    #[msg(exec)]
    fn execute(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        msgs: Vec<CosmosMsg<Msg>>,
    ) -> Result<Response, Self::Error>;
}

fn main() {}

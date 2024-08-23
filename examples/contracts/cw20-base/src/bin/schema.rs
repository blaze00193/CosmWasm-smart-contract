use cosmwasm_schema::write_api;
use cw20_base::contract::sv::{ContractExecMsg, ContractQueryMsg, InstantiateMsg};
use sylvia::cw_std::Empty;

#[cfg(not(tarpaulin_include))]
fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ContractExecMsg<Empty, Empty>,
        query: ContractQueryMsg<Empty, Empty>,
    }
}

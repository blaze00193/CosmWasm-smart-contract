use cosmwasm_std::{Addr, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdError};

use sylvia::interface;

#[derive(
    serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, schemars::JsonSchema,
)]
pub struct QueryResult;

#[interface]
pub trait Interface {
    type Error: From<StdError>;

    #[msg(exec)]
    fn no_args_execution(&self, ctx: (DepsMut, Env, MessageInfo)) -> Result<Response, Self::Error>;

    #[msg(exec)]
    fn argumented_execution(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        addr: Addr,
        coef: Decimal,
        desc: String,
    ) -> Result<Response, Self::Error>;

    #[msg(query)]
    fn no_args_query(&self, ctx: (Deps, Env)) -> Result<QueryResult, Self::Error>;

    #[msg(query)]
    fn argumented_query(&self, ctx: (Deps, Env), user: Addr) -> Result<QueryResult, Self::Error>;
}

#[test]
fn messages_constructible() {
    let no_args_exec = InterfaceExecMsg::NoArgsExecution {};
    let _argumented_exec = InterfaceExecMsg::ArgumentedExecution {
        addr: Addr::unchecked("owner"),
        coef: Decimal::percent(10),
        desc: "Some description".to_owned(),
    };
    let no_args_query = InterfaceQueryMsg::NoArgsQuery {};
    let _argumented_query = InterfaceQueryMsg::ArgumentedQuery {
        user: Addr::unchecked("owner"),
    };

    // Ensure no extra variants are generated
    match no_args_exec {
        InterfaceExecMsg::NoArgsExecution {} => (),
        InterfaceExecMsg::ArgumentedExecution { .. } => (),
    }

    match no_args_query {
        InterfaceQueryMsg::NoArgsQuery {} => (),
        InterfaceQueryMsg::ArgumentedQuery { .. } => (),
    }
}

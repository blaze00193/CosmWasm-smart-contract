use crate::contract::GenericContract;
use cosmwasm_std::{CosmosMsg, Empty, Response, StdError, StdResult};
use cw1::{CanExecuteResp, Cw1};
use sylvia::types::{ExecCtx, QueryCtx};

impl<
        InstantiateT,
        Exec1T,
        Exec2T,
        Exec3T,
        Query1T,
        Query2T,
        Query3T,
        Sudo1T,
        Sudo2T,
        Sudo3T,
        MigrateT,
        FieldT,
    > Cw1
    for GenericContract<
        InstantiateT,
        Exec1T,
        Exec2T,
        Exec3T,
        Query1T,
        Query2T,
        Query3T,
        Sudo1T,
        Sudo2T,
        Sudo3T,
        MigrateT,
        FieldT,
    >
{
    type Error = StdError;
    type ExecC = Empty;
    type QueryC = Empty;

    fn execute(&self, _ctx: ExecCtx, _msgs: Vec<CosmosMsg>) -> StdResult<Response> {
        Ok(Response::new())
    }

    fn can_execute(
        &self,
        _ctx: QueryCtx,
        _sender: String,
        _msg: CosmosMsg,
    ) -> StdResult<CanExecuteResp> {
        Ok(CanExecuteResp::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::contract::sv::mt::CodeId;
    use crate::contract::{GenericContract, SvCustomMsg, SvCustomQuery};
    use cosmwasm_std::{CosmosMsg, Empty};
    use cw1::sv::mt::Cw1Proxy;
    use cw_multi_test::IntoBech32;
    use sylvia::multitest::App;

    #[test]
    fn proxy_methods() {
        let app = App::<cw_multi_test::BasicApp<SvCustomMsg, SvCustomQuery>>::custom(|_, _, _| {});
        let code_id = CodeId::<
            GenericContract<
                SvCustomMsg,
                SvCustomMsg,
                SvCustomMsg,
                SvCustomMsg,
                SvCustomMsg,
                SvCustomMsg,
                SvCustomMsg,
                SvCustomMsg,
                SvCustomMsg,
                SvCustomMsg,
                SvCustomMsg,
                String,
            >,
            _,
        >::store_code(&app);

        let owner = "owner".into_bech32();

        let contract = code_id
            .instantiate(SvCustomMsg {})
            .with_label("GenericContract")
            .with_admin(owner.as_str())
            .call(&owner)
            .unwrap();

        contract.execute(vec![]).call(&owner).unwrap();
        contract
            .can_execute("sender".to_owned(), CosmosMsg::Custom(Empty {}))
            .unwrap();
    }
}

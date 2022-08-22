use crate::error::ContractError;
use cosmwasm_std::{Addr, DepsMut, Empty, Env, MessageInfo, Response};

use cw1::Cw1;
use cw1::*;
use cw_storage_plus::Map;
use sylvia::contract;

pub struct Cw1WhitelistContract {
    members: Map<'static, Addr, Empty>,
}

impl Cw1 for Cw1WhitelistContract {
    type Error = ContractError;
    fn add_member(
        &self,
        (deps, _env, _info): (
            cosmwasm_std::DepsMut,
            cosmwasm_std::Env,
            cosmwasm_std::MessageInfo,
        ),
        member: String,
    ) -> Result<Response, Self::Error> {
        self.members
            .save(deps.storage, deps.api.addr_validate(&member)?, &Empty {})?;

        Ok(Response::new())
    }

    fn find_member(
        &self,
        (deps, _env): (cosmwasm_std::Deps, cosmwasm_std::Env),
        member: String,
    ) -> Result<FindMemberResponse, Self::Error> {
        let is_present = self
            .members
            .has(deps.storage, deps.api.addr_validate(&member)?);

        Ok(FindMemberResponse { is_present })
    }
}

#[contract(error=ContractError)]
#[messages(cw1 as Cw1)]
impl Cw1WhitelistContract {
    pub const fn new() -> Self {
        Self {
            members: Map::new("members"),
        }
    }

    #[msg(instantiate)]
    pub fn instantiate(
        &self,
        (deps, _env, _msg): (DepsMut, Env, MessageInfo),
        members: Vec<String>,
    ) -> Result<Response, ContractError> {
        for addr in members.into_iter() {
            self.members
                .save(deps.storage, deps.api.addr_validate(&addr)?, &Empty {})?;
        }

        Ok(Response::new())
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        from_binary, from_slice,
        testing::{mock_dependencies, mock_env, mock_info},
        to_binary,
    };

    use crate::contract::InstantiateMsg;

    use super::*;

    #[test]
    fn binary_serialize_instantiate() {
        let original_msg = InstantiateMsg {
            members: vec!["member1".to_owned(), "member2".to_owned()],
        };

        let serialized_msg = to_binary(&original_msg).unwrap();
        let serialized_msg: InstantiateMsg = from_binary(&serialized_msg).unwrap();

        assert_eq!(serialized_msg, original_msg);
    }

    #[test]
    fn slice_deserialize_instantiate() {
        let deserialized: InstantiateMsg =
            from_slice(br#"{"members": ["member", "some_member"]}"#).unwrap();
        assert_eq!(
            deserialized,
            InstantiateMsg {
                members: vec!["member".to_owned(), "some_member".to_owned()]
            }
        );
    }

    #[test]
    fn instantiate_empty() {
        let contract = Cw1WhitelistContract::new();
        let mut deps = mock_dependencies();
        let info = mock_info("anyone", &[]);

        contract
            .instantiate((deps.as_mut(), mock_env(), info), vec![])
            .unwrap();
    }

    #[test]
    fn instantiate() {
        let contract = Cw1WhitelistContract::new();
        let mut deps = mock_dependencies();
        let members = vec!["alice".to_owned(), "brian".to_owned(), "carol".to_owned()];
        let info = mock_info("anyone", &[]);

        contract
            .instantiate((deps.as_mut(), mock_env(), info), members)
            .unwrap();

        let resp = contract
            .find_member((deps.as_ref(), mock_env()), "brian".to_owned())
            .unwrap();

        assert!(resp.is_present);

        let resp = contract
            .find_member((deps.as_ref(), mock_env()), "alice".to_owned())
            .unwrap();

        assert!(resp.is_present);

        let resp = contract
            .find_member((deps.as_ref(), mock_env()), "carol".to_owned())
            .unwrap();

        assert!(resp.is_present);

        let resp = contract
            .find_member((deps.as_ref(), mock_env()), "someone".to_owned())
            .unwrap();

        assert!(!resp.is_present);
    }

    #[test]
    fn add_member() {
        let contract = Cw1WhitelistContract::new();
        let mut deps = mock_dependencies();
        let members = vec!["alice".to_owned(), "brian".to_owned(), "carol".to_owned()];
        let info = mock_info("anyone", &[]);

        contract
            .instantiate((deps.as_mut(), mock_env(), info.clone()), members)
            .unwrap();

        contract
            .add_member((deps.as_mut(), mock_env(), info), "denis".to_owned())
            .unwrap();

        let resp = contract
            .find_member((deps.as_ref(), mock_env()), "denis".to_owned())
            .unwrap();

        assert!(resp.is_present);
    }
}
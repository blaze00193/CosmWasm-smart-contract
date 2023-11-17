use cosmwasm_std::{CosmosMsg, Response, StdError, StdResult};
use serde::{Deserialize, Serialize};
use sylvia::types::{ExecCtx, QueryCtx};
use sylvia::{interface, schemars};

#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, sylvia::schemars::JsonSchema, Debug, Default,
)]
pub struct CanExecuteResp {
    pub can_execute: bool,
}

#[interface]
pub trait Cw1 {
    type Error: From<StdError>;

    /// Execute requests the contract to re-dispatch all these messages with the
    /// contract's address as sender. Every implementation has it's own logic to
    /// determine in
    #[msg(exec)]
    fn execute(&self, ctx: ExecCtx, msgs: Vec<CosmosMsg>) -> Result<Response, Self::Error>;

    /// Checks permissions of the caller on this proxy.
    /// If CanExecute returns true then a call to `Execute` with the same message,
    /// from the given sender, before any further state changes, should also succeed.
    #[msg(query)]
    fn can_execute(
        &self,
        ctx: QueryCtx,
        sender: String,
        msg: CosmosMsg,
    ) -> StdResult<CanExecuteResp>;
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coins, from_json, to_json_binary, BankMsg};

    #[test]
    fn execute() {
        let original = super::sv::ExecMsg::Execute {
            msgs: vec![BankMsg::Send {
                to_address: "receiver".to_owned(),
                amount: coins(10, "atom"),
            }
            .into()],
        };

        let serialized = to_json_binary(&original).unwrap();
        let deserialized = from_json(serialized).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn execute_from_json() {
        let deserialized = from_json(br#"{"execute": { "msgs": [] }}"#).unwrap();
        assert_eq!(super::sv::ExecMsg::Execute { msgs: vec![] }, deserialized);
    }

    #[test]
    fn query() {
        let original = super::sv::QueryMsg::CanExecute {
            sender: "sender".to_owned(),
            msg: BankMsg::Send {
                to_address: "receiver".to_owned(),
                amount: coins(10, "atom"),
            }
            .into(),
        };

        let serialized = to_json_binary(&original).unwrap();
        let deserialized = from_json(serialized).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn query_from_json() {
        let deserialized = from_json(
            br#"{"can_execute": {
                "sender": "address",
                "msg": {
                    "bank": {
                        "send": {
                            "to_address": "receiver",
                            "amount": [
                                {
                                    "amount": "10",
                                    "denom": "atom"
                                }
                            ]
                        }
                    }
                }
            }}"#,
        )
        .unwrap();
        assert_eq!(
            super::sv::QueryMsg::CanExecute {
                sender: "address".to_owned(),
                msg: BankMsg::Send {
                    to_address: "receiver".to_owned(),
                    amount: coins(10, "atom"),
                }
                .into()
            },
            deserialized
        );
    }
}

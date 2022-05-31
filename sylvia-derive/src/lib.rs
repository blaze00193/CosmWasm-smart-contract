use input::{ImplInput, TraitInput};
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::fold::Fold;
use syn::{parse_macro_input, parse_quote, Ident, ItemImpl, ItemTrait, Path};

pub(crate) mod check_generics;
mod input;
mod message;
mod parser;
mod strip_generics;
mod strip_input;

use strip_input::StripInput;

pub(crate) fn crate_module() -> Path {
    use proc_macro_crate::{crate_name, FoundCrate};

    match crate_name("sylvia").expect("silvia is not found in Cargo.toml") {
        FoundCrate::Itself => parse_quote!(sylvia),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, proc_macro2::Span::mixed_site());
            parse_quote!(#ident)
        }
    }
}

/// Macro generating messages from contract trait.
///
/// ## Example usage
/// ```ignore
/// # use cosmwasm_std::Response;
///
/// # struct Ctx;
/// # struct Error;
///
/// # #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, schemars::JsonSchema)]
/// # struct Member;
///
/// # #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, schemars::JsonSchema)]
/// # struct AdminQueryResponse;
///
/// # #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, schemars::JsonSchema)]
/// # struct MemberQueryResponse;
///
/// #[sylvia::interface(module=msg)]
/// trait Cw4 {
///     type Error: From<StdError>;
///
///     #[msg(exec)]
///     fn update_admin(&self, ctx: (DepsMut, Env, MessageInfo), admin: Option<String>) -> Result<Response, Self::Error>;
///
///     #[msg(exec)]
///     fn update_members(&self, ctx: (DepsMut, Env, MessageInfo), remove: Vec<String>, add: Vec<Member>)
///         -> Result<Response, Self::Error>;
///
///     #[msg(query)]
///     fn admin(&self, ctx: (Deps, Env)) -> Result<AdminQueryResponse, Error>;
///
///     #[msg(query)]
///     fn member(&self, ctx: (Deps, Env), addr: String, at_height: Option<u64>) -> Result<MemberQueryResponse, Error>;
/// }
/// ```
///
/// This would generate output like:
///
/// ```ignore
/// pub mod msg {
///     # #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, schemars::JsonSchema)]
///     # struct Member;
///
///     #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, schemars::JsonSchema)]
///     #[serde(rename_all = "snake_case")]
///     pub enum ExecMsg {
///         UpdateAdmin { admin: Option<String> },
///         UpdateMembers {
///             remove: Vec<String>,
///             add: Vec<Member>,
///         },
///         AddHook { addr: String },
///         RemoveHook { addr: String },
///     }
///
///     impl ExecMsg {
///         pub fn dispatch<C: Cw4>(contract: &C, ctx: (DepsMut, Env, MessageInfo))
///             -> Result<Response, C::Error>
///         {
///             // Some dispatching implementation
///         }
///     }
/// }
///
/// And similar `Query` structure for handling queries.
/// ```
///
/// ## Parameters
///
/// `interface` attribute takes optional parameters:
/// * `module` - defines module name, where all generated messages would be encapsulated; no
/// additional module would be created if not provided
///
/// ## Attributes
///
/// Messages structures are generated basing on interface trait method attributed with
/// `#[msg(msg_type, ...)`. Msg attribute takes as its first argument type of message it is
/// supposed to handle:
///   * `exec` - this is execute message variant
///   * `query` - this is query message variant
///
/// For now, `#[msg(...)]` attribute doesn't support any additional data on `#[interface]`
/// elements, but it may be extended in future.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn interface(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as parser::InterfaceArgs);
    let input = parse_macro_input!(item as ItemTrait);

    let expanded = TraitInput::new(&attrs, &input).process();
    let input = StripInput.fold_item_trait(input);

    let expanded = quote! {
        #input

        #expanded
    };

    TokenStream::from(expanded)
}

/// Macro generating messages from contract impl block.
///
/// ## Example usage
/// ```ignore
/// # use cosmwasm_std::Response;
///
/// # struct Ctx;
/// # struct Error;
///
/// # #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, schemars::JsonSchema)]
/// # struct Cw4Group;
///
/// #[cw_derive::contract(module=msg)]
/// impl Cw4Group {
///     #[msg(instantiate, name="Instantiate")]
///     fn instantiate(&self, ctx: (DepsMut, Env, MessageInfo), admin: Option<String>)
///         -> Result<Response, Error>;
/// }
/// ```
///
/// This would generate output like:
///
/// ```ignore
/// pub mod msg {
///     # #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, schemars::JsonSchema)]
///     # struct Cw4Group;
///
///     #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, schemars::JsonSchema)]
///     #[serde(rename_all = "snake_case")]
///     pub struct Instantiate {
///         admin: Option<String>,
///     }
///
///     impl Instantiate {
///         fn dispatch(contract: &Cw4Group, ctx: (DepsMut, Env, MessageInfo), admin: Option<String>)
///             -> Result<Response, Error>
///         {
///             contract.instantiate(ctx, admin)
///         }
///     }
/// }
/// ```
///
/// ## Parameters
///
/// `contract` attribute takes optional parameters:
/// * `module` - defines module name, where all generated messages would be encapsulated; no
/// additional module would be created if not provided
///
/// ## Attributes
///
/// Messages structures are generated basing on specific implemented methods attributed with
/// `#[msg(msg_type, ...)`. Msg attribute takes as its first argument type of message it is
/// supposed to handle:
/// * `instantiate` - this is instantiation message handler. There should be always exactly one
/// handler for this kind of message.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as parser::ContractArgs);
    let input = parse_macro_input!(item as ItemImpl);

    let expanded = ImplInput::new(&attrs, &input).process();
    let input = StripInput.fold_item_impl(input);

    let expanded = quote! {
        #input

        #expanded
    };

    TokenStream::from(expanded)
}

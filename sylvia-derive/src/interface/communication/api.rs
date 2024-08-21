use crate::crate_module;
use crate::parser::variant_descs::AsVariantDescs;
use crate::parser::{Custom, MsgType};
use crate::types::associated_types::{AssociatedTypes, ItemType};
use crate::types::msg_variant::MsgVariants;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemTrait;

/// Emits `InterfaceMessagesApi` trait.
///
/// The `InterfaceMessagesApi` is a helper trait to access messages generated by the `interface`
/// macro.
/// It ease the dispatch of generic types.
pub struct Api<'a> {
    source: &'a ItemTrait,
    custom: &'a Custom,
    associated_types: &'a AssociatedTypes<'a>,
}

impl<'a> Api<'a> {
    pub fn new(
        source: &'a ItemTrait,
        custom: &'a Custom,
        associated_types: &'a AssociatedTypes<'a>,
    ) -> Self {
        Self {
            source,
            custom,
            associated_types,
        }
    }

    pub fn emit(&self) -> TokenStream {
        let sylvia = crate_module();
        let Self {
            source,
            custom,
            associated_types,
        } = self;

        let where_clause = &self.associated_types.as_where_clause();
        let custom_query = custom.query_or_default();
        let interface_name = &source.ident;
        let generics: Vec<_> = associated_types
            .without_error()
            .map(ItemType::as_name)
            .collect();
        let exec_variants = MsgVariants::new(
            source.as_variants(),
            MsgType::Exec,
            &generics,
            &source.generics.where_clause,
        );
        let query_variants = MsgVariants::new(
            source.as_variants(),
            MsgType::Query,
            &generics,
            &source.generics.where_clause,
        );
        let sudo_variants = MsgVariants::new(
            source.as_variants(),
            MsgType::Sudo,
            &generics,
            &source.generics.where_clause,
        );

        let exec_generics = &exec_variants.used_generics();
        let query_generics = &query_variants.used_generics();
        let sudo_generics = &sudo_variants.used_generics();

        let phantom = if !generics.is_empty() {
            quote! {
                _phantom: std::marker::PhantomData<( #(#generics,)* )>,
            }
        } else {
            quote! {}
        };

        quote! {
            pub trait InterfaceMessagesApi {
                type Exec;
                type Query;
                type Sudo;
                type Querier<'querier>;
            }

            impl<Contract: #interface_name> InterfaceMessagesApi for Contract {
                type Exec = ExecMsg < #(<Contract as #interface_name >:: #exec_generics,)* >;
                type Query = QueryMsg < #(<Contract as #interface_name >:: #query_generics,)* >;
                type Sudo = SudoMsg < #(<Contract as #interface_name >:: #sudo_generics ,)* >;
                type Querier<'querier> = #sylvia ::types::BoundQuerier<'querier, #custom_query, Contract >;
            }

            impl<'sv_iface_msg_api, Error, #(#generics),*> InterfaceMessagesApi for dyn #interface_name < Error = Error, #(#generics = #generics,)* > + 'sv_iface_msg_api {
                type Exec = ExecMsg < #(#exec_generics,)* >;
                type Query = QueryMsg < #(#query_generics,)* >;
                type Sudo = SudoMsg < #(#sudo_generics,)* >;
                type Querier<'querier> = #sylvia ::types::BoundQuerier<'querier, #custom_query, Self >;
            }

            pub struct Api < #(#generics,)* > {
                #phantom
            }

            #[allow(deprecated)]
            impl < #(#generics,)* > #sylvia ::types::InterfaceApi for Api < #(#generics,)* > #where_clause {
                type Exec = ExecMsg < #(#exec_generics,)* >;
                type Query = QueryMsg < #(#query_generics,)* >;
                type Sudo = SudoMsg < #(#sudo_generics,)* >;
                type Querier<'querier, Contract> = #sylvia ::types::BoundQuerier<'querier, #custom_query, Contract >;
            }
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

mod error;

#[openbrush::contract]
pub mod contract_name {
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;
    use project_name::impls::trait_name::*;

    pub use crate::error::ContractError;

    #[ink(event)]
    pub struct ValueFlipped {
        #[ink(topic)]
        pub value: bool,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        trait_name: trait_name::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn try_new() -> Result<Self, ContractError> {
            Ok(Self::default())
        }
    }

    impl TraitName for Contract {}

    impl Internal for Contract {
        fn _emit_flip_event(&self, value: bool) {
            EmitEvent::<Contract>::emit_event(self.env(), ValueFlipped { value })
        }
    }
}

#[cfg(test)]
mod unit_tests;

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests;

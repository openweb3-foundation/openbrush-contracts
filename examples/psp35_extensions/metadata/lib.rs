#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp35 {
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp35::extensions::metadata::*,
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp35: psp35::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP35 for Contract {}

    impl PSP35Metadata for Contract {}

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn set_attribute(&mut self, id: Id, key: Vec<u8>, data: Vec<u8>) -> Result<(), PSP35Error> {
            self._set_attribute(&id, &key, &data)
        }
    }
}

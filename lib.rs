#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod my_psp34 {
    
    // imports from openbrush
	use openbrush::contracts::psp34::*;
	use openbrush::traits::Storage;
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
    	#[storage_field]
		psp34: psp34::Data,
        name: String,
    }
    
    // Section contains default implementation without any modifications
	impl PSP34 for Contract {}
     
    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			_instance
        }
    }
}
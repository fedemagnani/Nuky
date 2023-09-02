// We create a payment splitter contract: it collects money and then it spreads this money across different addresses (AccountId)

// Firstly, we give up on the standard library 
#![cfg_attr(not(feature = "std"), no_std, no_main)]

// Then, we import the PaymentSplitter smart contract from openbrush and we declare that the following module is actually a smart contract. Indeed, a smart contract is a module:
    // - state variables are saved into a struct under macros #[ink(storage)] #[derive(Default, Storage)]. The key of the smart contract is always anticipated by the #[storage_field] macro
    // - methods are inside the implementation of the struct. One method must be anticipated by the #[ink(constructor)] macro and is equivalent to the constructor
#[openbrush::implementation(PaymentSplitter)]
// By importing this smart contract, we are allowed to create a state variable (key of the struct) whose value is the imported smart contract data
#[openbrush::contract]
mod spread_money {
    // We import the Storage trait where we're going to save our state variables
    use openbrush::{traits::Storage, contracts::payment_splitter::{self, PaymentSplitterError}};

    // We create an event which is going to be emitted in the "shoot_money" method
    #[ink(event)]
    pub struct MoneyShot {
        #[ink(topic)]
        pub from: AccountId,
        // #[ink(topic)]
        // pub amount: Balance,
    }

    #[ink(storage)] 
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        //notice that the name is the same that you put in features in Cargo.toml
        splitter: payment_splitter::Data
    }
    impl Contract {
        #[ink(constructor)]
        pub fn at_deploy(payees_and_shares: Vec<(AccountId, u128)>) -> Self {
            //Notice that Contract inherited the Default trait
            let mut instance = Self::default();
            // we update the state variables of the payment splitter using its dedicated method accepting mutable reference as input
            payment_splitter::Internal::_init(&mut instance,payees_and_shares).expect("failed to init payment splitter");
            instance
        }

        // Any other smart contract method has the macro #[ink(message)] and is public
        #[ink(message)]
        pub fn shoot_money(&mut self)-> Result<(), PaymentSplitterError>{
            // We call the method of the payment splitter smart contract
            match payment_splitter::Internal::_release_all(self){
                Ok(_) => {
                    // We emit the event
                    self.env().emit_event(MoneyShot {
                        from: self.env().caller(),
                        // amount: amount,
                    });
                    Ok(())
                },
                Err(err) => Err(err)
            }
        }
    }

    
}
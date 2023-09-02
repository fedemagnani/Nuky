For importing specific smart contracts from open brush, you need to add the features with same name, snake_case, in Cargo.toml

A smart contract is a module:
    - state variables are saved into a struct under macros #[ink(storage)] #[derive(Default, Storage)]
    - methods are inside the implementation of the struct

Differently from Solidity, the variables belonging to a parent smart contract are not directly accessible. To access those variables. Indeed, the parent smart contract is actually saved in the storage of the child smart contract; moreover, to edit the variables of the parent smart contract, you need to use the methods of the parent smart contract taking the mutable reference as input parameter. 

Events are struct anticipated by macro #[ink(event)] and are emitted via self.env().emit_event(Evento)
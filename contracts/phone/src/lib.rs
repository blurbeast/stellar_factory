#![no_std]
// use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

// #[contract]
// pub struct Contract;

// #[contractimpl]
// impl Contract {
//     pub fn hello(env: Env, to: String) -> Vec<String> {
//         vec![&env, String::from_str(&env, "Hello"), to]
//     }
// }

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

mod test;


#[contract]
pub struct Phone;



#[contracttype]
pub enum PhoneKey{
    Owner,
    OwnerDetails,
}


#[contractimpl]
impl Phone {
    
    pub fn __init(env: Env, owner: Address, name: String) {
        if env.storage().instance().has(&PhoneKey::Owner) {}

        env.storage().instance().set(&PhoneKey::Owner, &owner);   
        env.storage().instance().set(&PhoneKey::OwnerDetails, &(owner, name));
    }


    pub fn create_new_phone_book(env: Env) {
        // env.deployer()
        // env.register(contract, constructor_args)
    }
}
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

use soroban_sdk::{contract, contractimpl, contracttype, xdr::Uint256, Address, Env, String, U256};

mod test;



#[contracttype]
pub enum PhoneBookKey {
    Owner,
    Name,
    Contact(String,),
}


#[contracttype]
#[derive(Debug)]
pub struct ContactInfo {
    pub name: String,
    pub mobile_number: String,
}

#[contract]
pub struct PhoneBook;


#[contractimpl]
impl PhoneBook {

    pub fn __init(env: &Env, owner: Address, name: String) {
        if env.storage().instance().has(&PhoneBookKey::Owner) {
            // throw
        }

        env.storage().instance().set(&PhoneBookKey::Owner, &owner);
        env.storage().instance().set(&PhoneBookKey::Name, &name);
    }


    pub fn add_contact(env: &Env, name: String, mobile_number: String) {

        let contact_info = ContactInfo {
            name: name.clone(),
            mobile_number
        };
        env.storage().instance().set(&PhoneBookKey::Contact(name), &contact_info);
    }

    pub fn get_contact(env: Env, name: String) -> Option<ContactInfo> {
        env.storage().instance().get(&PhoneBookKey::Contact(name))
    }

    pub fn remove_contact(env: Env, name: String) {
        // Verify caller is the owner
        let owner: Address = env.storage().instance().get(&PhoneBookKey::Owner)
            .expect("Contract not initialized");
        owner.require_auth();

        let is_exist:bool = env.storage().instance().has(&PhoneBookKey::Contact(name.clone()));
        if !is_exist {
            
        }
        
        env.storage().instance().remove(&PhoneBookKey::Contact(name));
    }
}

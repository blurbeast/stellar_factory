#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, String, Val, Vec};

mod test;
use phone_book::PhoneBookClient;

#[contract]
pub struct Phone;

#[contracttype]
pub enum PhoneKey {
    Owner,
    OwnerDetails,
    PhoneBookName(String),
}

#[contracttype]
pub struct ContactInfo {
    pub name: String,
    pub no: String,
}

#[contractimpl]
impl Phone {
    pub fn __init(env: Env, owner: Address, name: String) {
        if env.storage().instance().has(&PhoneKey::Owner) {}

        env.storage().instance().set(&PhoneKey::Owner, &owner);
        env.storage()
            .instance()
            .set(&PhoneKey::OwnerDetails, &(owner, name));
    }

    pub fn create_new_phone_book(
        env: Env,
        phone_book_name: String,
        salt: BytesN<32>,
        wasm_hash: BytesN<32>,
        constructor_args: Vec<Val>,
    ) {
        let owner: Address = env.storage().instance().get(&PhoneKey::Owner).expect("msg");
        owner.require_auth();
        let phone_book_address: Address = env
            .deployer()
            .with_address(owner.clone(), salt)
            .deploy_v2(wasm_hash, constructor_args);

        // save it
        env.storage().instance().set(
            &PhoneKey::PhoneBookName(phone_book_name),
            &phone_book_address,
        );
    }

    pub fn add_new_contact(env: Env, phone_book_name: String, contact_info: ContactInfo) {
        let owner: Address = env.storage().instance().get(&PhoneKey::Owner).expect("msg");
        owner.require_auth();

        let phone_book_address: Address = env
            .storage()
            .instance()
            .get(&PhoneKey::PhoneBookName(phone_book_name))
            .expect("msg");

        let phone_book_instance: PhoneBookClient = PhoneBookClient::new(&env, &phone_book_address);

        phone_book_instance.add_contact(&contact_info.name, &contact_info.no);
    }

    pub fn get_contact(
        env: Env,
        phone_book_name: String,
        contact_name: String,
    ) -> Option<phone_book::ContactInfo> {
        let owner: Address = env.storage().instance().get(&PhoneKey::Owner).expect("msg");
        owner.require_auth();

        let phone_book_address: Address = env
            .storage()
            .instance()
            .get(&PhoneKey::PhoneBookName(phone_book_name))
            .expect("msg");

        let phone_book_instance: PhoneBookClient = PhoneBookClient::new(&env, &phone_book_address);

        phone_book_instance.get_contact(&contact_name)
    }
}

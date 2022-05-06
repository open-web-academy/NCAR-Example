use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
    PromiseResult, Gas, require, serde_json::json
};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Item {
    address: String,
    name: String,
    price: u64,
    stock: u64,
    enabled: bool
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub records: LookupMap<String, Item>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init_contract(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id
        )
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            owner_id,
            records: LookupMap::new(b"a".to_vec()),
        };

        //return the Contract object
        this
    }

    // Guardar producto
    pub fn set_products(&mut self, address:String, name:String, price:u64, stock:u64) -> Item {
        //validate sender has permition of ROLE_SET_PRODUCT
        //assert_eq!(self.access.has_role(&ROLE_SET_PRODUCT.to_string(), &env::signer_account_id()), true, "401");

        let item = Item {
            address : address.to_string(),
            name : name.to_string(),
            price : price,
            stock : stock,
            enabled : true
        };

        self.records.insert(&address, &item);

        env::log(
            json!(item.clone())
            .to_string()
            .as_bytes(),
        );

        item
    }

    // Eliminar producto
    pub fn delete_products(&mut self, address:String) {
        //validate sender has permition of ROLE_DELETE_PRODUCT
        //assert_eq!(self.access.has_role(&ROLE_DELETE_PRODUCT.to_string(), &env::signer_account_id()), true, "401");
       
        // Use env::log to record logs permanently to the blockchain!
        let delete_product = self.records.get(&address);

        self.records.remove(&address);

        env::log(format!("delete_products '{}' ", address).as_bytes());
   }

    // Consultar producto
    pub fn get_products(&self, address:String) -> Option<Item>{
        self.records.get(&address)
   }

}
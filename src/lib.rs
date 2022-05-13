use std::borrow::Borrow;

// use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::collections::LookupMap;
use near_sdk::env::log_str;
// use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, require, serde_json::json, AccountId, Balance, CryptoHash, Gas,
    PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};

// use near_sdk::{bindgen, AccountId, PanicOnDefault};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Item {
    address: String,
    name: String,
    price: u64,
    stock: u64,
    cid: String,
    enabled: bool,
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
        Self::new(owner_id)
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
    pub fn set_products(
        &mut self,
        address: String,
        name: String,
        price: u64,
        stock: u64,
        cid: String,
    ) -> Item {
        //validate sender has permition of ROLE_SET_PRODUCT
        let item = Item {
            address: address.to_string(),
            name: name.to_string(),
            price: price,
            stock: stock,
            cid: cid.to_string(),
            enabled: true,
        };

        log_str(
            &json!({
            "address": item.address.clone(),
            "name": item.name.clone(),
            "price": item.price.clone(),
            "stock": item.stock.clone(),
            "cid": item.cid.clone(),
            "enabled": item.enabled.clone()
            })
            .to_string(),
        );

        self.records.insert(&address, &item);
        item
    }

    // Eliminar producto
    pub fn delete_products(&mut self, address: String) {
        //validate sender has permition of ROLE_DELETE_PRODUCT

        // Use env::log to record logs permanently to the blockchain!
        let delete_product = match self.records.get(&address) {
            Some(item) => item.clone(),
            None => Item {
                address: "".to_string(),
                name: "".to_string(),
                price: 0,
                stock: 0,
                cid: "".to_string(),
                enabled: false,
            },
        };

        log_str(
            &json!({
            "address": delete_product.address.clone(),
            "name": delete_product.name.clone(),
            "price": delete_product.price.clone(),
            "stock": delete_product.stock.clone(),
            "cid": delete_product.cid.clone(),
            "enabled": delete_product.enabled.clone()
            })
            .to_string(),
        );

        self.records.remove(&address);
    }

    // Consultar producto
    pub fn get_products(&self, address: String) -> Option<Item> {
        self.records.get(&address)
    }
}

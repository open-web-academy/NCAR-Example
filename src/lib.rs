/*  
    Dia 1 : Interacción con contratos inteligentes.
  
    - ☑️ Clona el repositorio de Github, compila y despliega el contrato. 
    - ☑️Crea la estructura de archivos para tu contrato inteligente, es decir, los archivos migrate.rs, internals.rs, enumerations.rs y los que consideres necesarios.
    -☑️ Implementa las buenas prácticas recomendadas por el Protocolo de NEAR para el lenguaje de programación Rust.
    -☑️  Corrige el archivo **Cargo.toml** para optimizar el peso del archivo compilado.
    - ¡Compila y Despliega tu contrato para realizar las pruebas necesarias y seguir añadiendo las herramientas para escalabilidad y mantenimiento para tu DApp!

*/


/*  
    Estructura de archivos de un contrato:

    lib.rs: estructura de contrato y metodos de inicializacion

*/

// use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::collections::{ LookupMap };
// use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
// use near_sdk::{
//     env, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
//     PromiseResult, Gas, require, serde_json::json
// };
use near_sdk::{
   near_bindgen, AccountId,  PanicOnDefault 
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
        let item = Item {
            address : address.to_string(),
            name : name.to_string(),
            price : price,
            stock : stock,
            enabled : true
        };

        self.records.insert(&address, &item);

        item
    }

    // Eliminar producto
    pub fn delete_products(&mut self, address:String) {
        //validate sender has permition of ROLE_DELETE_PRODUCT
       
        // Use env::log to record logs permanently to the blockchain!
        let _delete_product = self.records.get(&address); // none or some

        self.records.remove(&address);
   }

    // Consultar producto
    pub fn get_products(&self, address:String) -> Option<Item>{
        self.records.get(&address)
   }

}
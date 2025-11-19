#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map, String};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // Submit feedback
    pub fn submit_feedback(env: Env, address: Address, feedback: String) {
        let key = symbol_short!("feedback");
        let mut feedback_map: Map<Address, String> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));
        
        feedback_map.set(address.clone(), feedback);
        env.storage().instance().set(&key, &feedback_map);
    }

    // Get feedback
    pub fn get_feedback(env: Env, address: Address) -> String {
        let key = symbol_short!("feedback");
        let feedback_map: Map<Address, String> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));
        
        feedback_map.get(address).unwrap_or(String::from_str(&env, ""))
    }
}

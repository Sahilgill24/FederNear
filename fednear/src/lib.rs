use std::{clone, default, path::Prefix};

use near_sdk::{
    env, log, near,
    store::{IterableMap, LookupMap},
    AccountId, NearToken, Promise,
};

const DEFAULT_AMOUNT: u128 = 10000000;

#[near(contract_state)]
pub struct Aggregator {
    pub registry: Vec<AccountId>,
    pub aggregation: IterableMap<u64, u64>,
    pub to: AccountId,
    pub amount: NearToken,
    pub federatedaverage: u64,
}

impl Default for Aggregator {
    fn default() -> Self {
        Self {
            registry: Vec::new(),
            aggregation: IterableMap::new(b"a".to_vec()),
            to: env::current_account_id(),
            amount: NearToken::from_yoctonear(DEFAULT_AMOUNT),
            federatedaverage: 0,
        }
    }
}

#[near]
impl Aggregator {
    // node registers and now has to pay
    pub fn noderegistation(&mut self, node: AccountId) {
        self.registry.push(node);
        let receiver = clone::Clone::clone(&self.to);
        Promise::new(receiver).transfer(self.amount);
    }

    
    #[private]
    pub fn decryption() {}

    
    pub fn Aggregation()-> u128 {
        12
    }
}

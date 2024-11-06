use std::{clone, default, path::Prefix};
use near_sdk::{
    env, log, near,
    store::{IterableMap, LookupMap},
    AccountId, NearToken, Promise,
    borsh::{self, BorshDeserialize, BorshSerialize},
};
use num_bigint::BigUint;
use num_traits::Num;

#[near(serializers = [json, borsh])]
pub struct CommitmentInfo {
    commitment: Vec<u8>,    // The commitment value c
    value: u64,            // The original value x
    randomness: Vec<u8>,   // The random value r used
}

#[near(contract_state)]
pub struct Aggregator {
    pub registry: Vec<AccountId>,
    pub aggregation: IterableMap<u64, u64>,
    pub to: AccountId,
    pub amount: NearToken,
    pub commitments: LookupMap<AccountId, CommitmentInfo>,
    // Parameters for Pederson commitment
    pub g: Vec<u8>,        // Generator
    pub h: Vec<u8>,        // Second generator
    pub q: Vec<u8>,        // Group order
}

impl Default for Aggregator {
    fn default() -> Self {
        Self {
            registry: Vec::new(),
            aggregation: IterableMap::new(b"a".to_vec()),
            to: env::current_account_id(),
            amount: NearToken::from_yoctonear(100000),
            commitments: LookupMap::new(b"c"),
            g: Vec::new(),
            h: Vec::new(),
            q: Vec::new(),
        }
    }
}

#[near]
impl Aggregator {
    pub fn noderegistation(&mut self, node: AccountId) {
        self.registry.push(node);
        let receiver = clone::Clone::clone(&self.to);
        Promise::new(receiver).transfer(self.amount);
    }
    
    #[private]
    pub fn decryption(&self) -> Option<u64> {
        // We'll sum up all values and random factors
        let mut total_value: u64 = 0;
        let mut total_r = BigUint::from(0u32);
        let q = BigUint::from_bytes_be(&self.q);
        
        // Iterate through all commitments
        for commitment_info in self.commitments.get(& self.commitments).iter() {
            // Add the original value
            total_value = total_value.checked_add(commitment_info.value)?;
            
            // Add the random value (modulo q)
            let r = BigUint::from_bytes_be(&commitment_info.randomness);
            total_r = (total_r + r) % &q;
        }

        // The final aggregated commitment can be verified using:
        // c_total = g^(sum of values) * h^(sum of r) mod q
        
        // Log the decrypted total
        log!("Decrypted total value: {}", total_value);
        log!("Total randomness used: {}", total_r.to_string());
        
        Some(total_value)
    }

    // Helper function to store a new commitment
    pub fn store_commitment(&mut self, 
        account: AccountId, 
        commitment: Vec<u8>, 
        value: u64, 
        randomness: Vec<u8>
    ) {
        let commitment_info = CommitmentInfo {
            commitment,
            value,
            randomness,
        };
        self.commitments.insert(account, commitment_info);
    }
    
}

// Add tests

use candid::{CandidType, Deserialize};
use ic_cdk_macros::{update, query};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Default)]
struct Wallet {
    owner: String,
    balances: HashMap<String, u64>, // Key: Token identifier, Value: Token balance
}

impl Wallet {
    // Constructor to initialize a new Wallet with the owner's ID
    fn new(owner: String) -> Self {
        Self {
            owner,
            balances: HashMap::new(),
        }
    }

    // Method to send tokens to another address
    fn send_tokens(&mut self, to: String, amount: u64) {
        // Ensure the owner has enough balance to send
        let owner_balance = self.balances.entry(self.owner.clone()).or_insert(0);
        if *owner_balance < amount {
            ic_cdk::trap("Insufficient balance");
        }
        
        // Subtract the amount from the owner's balance
        *owner_balance -= amount;
        
        // Add the amount to the recipient's balance
        let recipient_balance = self.balances.entry(to).or_insert(0);
        *recipient_balance += amount;
    }

    // Method to receive tokens from another address
    fn receive_tokens(&mut self, from: String, amount: u64) {
        // Add the amount to the owner's balance
        let owner_balance = self.balances.entry(self.owner.clone()).or_insert(0);
        *owner_balance += amount;

        // Subtract the amount from the sender's balance
        let sender_balance = self.balances.entry(from).or_insert(0);
        if *sender_balance < amount {
            ic_cdk::trap("Sender has insufficient balance");
        }
        *sender_balance -= amount;
    }

    // Method to get the current balance of the owner
    fn get_balance(&self) -> u64 {
        // Return the balance of the owner
        *self.balances.get(&self.owner).unwrap_or(&0)
    }
}

// Update method to send tokens from the wallet
#[update]
fn send(to: String, amount: u64) {
    let mut wallet = Wallet::new(ic_cdk::caller().to_string());
    wallet.send_tokens(to, amount);
}

// Update method to receive tokens into the wallet
#[update]
fn receive(from: String, amount: u64) {
    let mut wallet = Wallet::new(ic_cdk::caller().to_string());
    wallet.receive_tokens(from, amount);
}

// Query method to get the current balance of the wallet
#[query]
fn balance() -> u64 {
    let wallet = Wallet::new(ic_cdk::caller().to_string());
    wallet.get_balance()
}

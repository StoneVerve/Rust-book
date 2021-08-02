use serde::{Deserialize, Serialize};
use serde_json::Result;

// Implementation of a simple transaction in which one user sends an amount 
// to another user
struct Transaction {
	from: String,
	to: String,
	amount: u128,
}

// Creates a new transaction
//
fn create_transaction(sender: String, recipient: String, _amount: u128) {
	return Transaction {
		from: sender,
		to: recipient,
		amount: _amount,
	};
}


// Returns a string wich represents a transaction in json format
fn transaction_to_json(transaction: Transaction) {
	return serde_json::to_string(&transaction);
}
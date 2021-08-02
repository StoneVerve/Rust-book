use sha2::Sha256
use hex_literal::hex;

cons DIRECTION_ZERO ="0x0000000000000000000000000000000000000000000000000000000000000000";

// Implementation of a simple block of a block chain
// 
struct Block {
	block_number: u64,
	transaction: Transaction,
	timestamp: u128,
	previous_block: bytes,
	nonce: u64,
}

// Small function to create a new transaction
// 
fn create_transaction(sender: String, recipient: String, _amount: u128) {
	return Transaction {
		from: sender,
		to: recipient,
		amount: _amount,
		nonce: u64,
	};
}

// Generats de genesis block, which is the first block of a block chain.
// The previous_block is a reference to a non-existent block
fn crate_genesis_block(){
	return Block {
		block_number: 0,
		transaction: create_transaction("", "", 0),
		timestamp: 0,
		previous_block: hex!(DIRECTION_ZERO),
		nonce = 0,
	};
}
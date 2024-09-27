use std::collections::BTreeMap;

use num::{CheckedAdd, One, Zero};

use crate::Config;


/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T:Config> Pallet<T>

{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		let block_num = self.block_number.checked_add(&T::BlockNumber::one()).unwrap();
		self.block_number = block_num

	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		// self.nonce.entry(who.clone()).and_modify(|e| { 
		// 	e.checked_add(&Nonce::one()).unwrap();
		// }).or_insert(Nonce::one());
		
		let nonce: T::Nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        let new_nonce = nonce + T::Nonce::one();
        self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod test {
	use crate::types::*;
	#[test]
	fn init_system() {
		let mut system : super::Pallet<TestConfig> = super::Pallet::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());
		system.inc_nonce(&"bob".to_string());
		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(&"alice".to_string()), Some(&1));
		assert_eq!(system.nonce.get(&"bob".to_string()), Some(&1));
	}
}

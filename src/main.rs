mod balances;
mod system;
mod types;
use types::*;
use std::fmt::Debug;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}
impl Config for Runtime {
	type AccountId = String;
	type BlockNumber = u32;
	type Nonce = u32;
	type Balance = u128;
}

impl Runtime<> {
	/// Creates a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();
	runtime.balances.set_balance(&"alice".to_string(), 100);

	// start emulating a block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	runtime.system.inc_nonce(&"alice".to_string());
	let _ = runtime
		.balances
		.transfer(&"alice".to_owned(), &"bob".to_owned(), 30)
		.map_err(|e| eprintln!("{}", e));

	runtime.system.inc_nonce(&"alice".to_string());
	let _ = runtime
		.balances
		.transfer(&"alice".to_owned(), &"charlie".to_owned(), 20)
		.map_err(|e| eprintln!("{}", e));

	println!("{:#?}", runtime);
}

mod balances;
mod system;
mod types;
mod support;
use std::fmt::Debug;
use types::*;
use crate::support::Dispatch;

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
impl crate::support::Dispatch for Runtime {
    type Caller = AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    //
    // Dispatch allows us to identify which underlying module call we want to execute.
    // Note that we extract the `caller` from the extrinsic, and use that information
    // to determine who we are executing the call on behalf of.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
			RuntimeCall::BalancesTransfer { to, amount } => {
				self.balances.transfer(&caller, &to, amount)?;
				Ok(())
			}
		}
    }
}

impl Runtime {
	/// Creates a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
	 // Execute a block of extrinsics. Increments the block number.
	 fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {      
		self.system.inc_block_number();
		assert_eq!(self.system.block_number(),block.header.block_number,"Incorrect Block Number");
		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _res = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
		}
        Ok(())
    }
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    BalancesTransfer { to: types::AccountId, amount: types::Balance },
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
	
	// second transaction
	runtime.system.inc_nonce(&"alice".to_string());
	let _ = runtime
		.balances
		.transfer(&"alice".to_owned(), &"charlie".to_owned(), 20)
		.map_err(|e| eprintln!("{}", e));

	println!("{:#?}", runtime);
}

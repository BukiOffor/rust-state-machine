use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

use crate::{support::DispatchResult, Config};

//use crate::types::*;
#[derive(Debug)]
pub struct Pallet<T: Config> {
	// we do this to specify which type from which trait, 
	// because what if there is another trait that the type implements that has the same type Name
	balances: BTreeMap<<T as Config>::AccountId, T::Balance>, 
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum EntryPoint<T: Config> {
	//Transfer(T::AccountId,T::Balance)
	Transfer { to: T::AccountId, amount: T::Balance },
}

/// Implementation of the dispatch logic, mapping from `BalancesCall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
	//we are pulling the AccountId type in the config trait
	type Caller = T::AccountId;
	type Call = EntryPoint<T>;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		call: Self::Call,
	) -> crate::support::DispatchResult {
		match call {
			EntryPoint::Transfer { to, amount } => self.transfer(&caller, &to, amount)?,
		}
		Ok(())
	}
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}
	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}
	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}
	pub fn transfer(
		&mut self,
		from: &T::AccountId,
		to: &T::AccountId,
		amount: T::Balance,
	) -> DispatchResult {
		let caller_balance = self.balance(&from);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.balances.insert(from.clone(), new_caller_balance);
		self.balances.insert(to.clone(), new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use crate::types::*;
	#[test]
	fn init_balances() {
		let mut balances: super::Pallet<TestConfig> = crate::balances::Pallet::new();
		assert_eq!(balances.balance(&"alice".to_owned()), 0);
		balances.set_balance(&"alice".to_owned(), 100);
		assert_eq!(balances.balance(&"alice".to_owned()), 100);
		assert_eq!(balances.balance(&"bob".to_owned()), 0);
	}
	#[test]
	fn test_transfer() {
		let mut balances: super::Pallet<TestConfig> = crate::balances::Pallet::new();
		assert_eq!(
			balances.transfer(&"alice".to_owned(), &"bob".to_owned(), 51),
			Err("Not enough funds.")
		);
		balances.set_balance(&"alice".to_owned(), 100);
		balances.set_balance(&"bob".to_owned(), 100);
		balances.transfer(&"alice".to_owned(), &"bob".to_owned(), 50).unwrap();
		assert_eq!(balances.balance(&"alice".to_owned()), 50);
		assert_eq!(balances.balance(&"bob".to_owned()), 150);
	}
}

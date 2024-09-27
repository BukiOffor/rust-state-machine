use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

//use crate::types::*;
#[derive(Debug)]
pub struct Pallet<AccountId,Balance> {
	balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId,Balance> Pallet<AccountId,Balance>
where
    AccountId: Ord + Clone,
    Balance: Zero + CheckedSub + CheckedAdd + Copy,
	{
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}
	pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
		self.balances.insert(who.clone(), amount);
	}
	pub fn balance(&self, who: &AccountId) -> Balance {
		*self.balances.get(who).unwrap_or(&Balance::zero())
	}
	pub fn transfer(
		&mut self,
		from: &AccountId,
		to: &AccountId,
		amount: Balance,
	) -> Result<(), &'static str> {
        let caller_balance = self.balance(&from);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(from.clone(), new_caller_balance);
        self.balances.insert(to.clone(), new_to_balance);

        Ok(())
	}
}

#[test]
fn init_balances() {
	let mut balances: Pallet<&str, u128> = crate::balances::Pallet::new();
	assert_eq!(balances.balance(&"alice"), 0);
	balances.set_balance(&"alice", 100);
	assert_eq!(balances.balance(&"alice"), 100);
	assert_eq!(balances.balance(&"bob"), 0);
}
#[test]
fn test_transfer() {
	let mut balances: Pallet<&str, u128> = crate::balances::Pallet::new();
	assert_eq!(
		balances.transfer(&"alice", &"bob", 51),
		Err("Not enough funds.")
	);
	balances.set_balance(&"alice", 100);
	balances.set_balance(&"bob", 100);
	balances.transfer(&"alice", &"bob", 50).unwrap();
	assert_eq!(balances.balance(&"alice"), 50);
	assert_eq!(balances.balance(&"bob"), 150);
}

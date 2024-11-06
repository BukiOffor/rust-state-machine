use core::fmt::Debug;
use std::{cmp::Ordering, collections::BTreeMap};

use crate::support::DispatchResult;

pub trait Config: crate::types::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	pub claims: BTreeMap<<T as Config>::Content, T::AccountId>,
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum EntryPoint<T: Config> {
    CreateClaim(T::Content),
    RevokeClaim(T::Content),
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
			EntryPoint::CreateClaim(claim) => self.create_claim(caller, claim)?,
            EntryPoint::RevokeClaim(claim) => self.revoke_claim(caller, claim)?
		}
		Ok(())
	}
}


impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}
	/// Get the owner (if any) of a claim.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim.to_owned())
	}
	/// Create a new claim on behalf of the `caller`.
	/// This function will return an error if someone already has claimed that content.
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.get_claim(&claim).is_some() {
			return Err("This claim has already been created")
		}
		self.claims.insert(claim, caller);
		Ok(())
	}

	/// Revoke an existing claim on some content.
	/// This function should only succeed if the caller is the owner of an existing claim.
	/// It will return an error if the claim does not exist, or if the caller is not the owner.
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let content = self.claims.get(&claim).ok_or("Claim does not exist")?;
		if content.cmp(&caller) != Ordering::Equal {
			return Err("Content does not match the caller")
		}
		self.claims.remove(&claim);
		Ok(())
	}
}



#[cfg(test)]
mod test {
	pub struct Test {}

	impl super::Config for Test {
		type Content = &'static str;
	}

	impl crate::types::Config for Test {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
		type Balance = u128;
	}
	#[test]
	fn basic_proof_of_existence() {
		let mut pallet = super::Pallet::<Test>::new();
		assert_eq!(pallet.get_claim(&"Hello, world!"), None);
		assert_eq!(pallet.create_claim("alice", "Hello, world!"), Ok(()));
		assert_eq!(pallet.get_claim(&"Hello, world!"), Some(&"alice"));
		assert_eq!(
			pallet.create_claim("bob", "Hello, world!"),
			Err("This claim has already been created")
		);
		assert_eq!(pallet.revoke_claim("alice", "Hello, world!"), Ok(()));
		assert_eq!(pallet.create_claim("bob", "Hello, world!"), Ok(()));
	}
}

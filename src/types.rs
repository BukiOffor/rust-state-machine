#![allow(dead_code)]
use std::{fmt::Debug, ops::AddAssign};

use num::{CheckedAdd, CheckedSub, One, Zero};

pub trait Config {
	type AccountId: Ord + Clone + Debug + ToString;
	type BlockNumber: Zero + One + AddAssign + Copy + CheckedAdd + CheckedSub + Debug;
	type Nonce: Zero + One + Copy + CheckedAdd + CheckedSub + Debug;
	type Balance: Zero + CheckedSub + CheckedAdd + Copy + AddAssign + One + Debug;
}

pub type AccountId = String;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Nonce = u32;
pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
pub type Header = crate::support::Header<BlockNumber>;
pub type Block = crate::support::Block<Header, Extrinsic>;
pub type Content = String;
pub type Signature<T> = Result<T, String>;

pub struct TestConfig;

impl super::Config for TestConfig {
	type AccountId = String;
	type BlockNumber = u32;
	type Nonce = u32;
	type Balance = u128;
}

/// This trait is not object safe
pub trait Signer<T: Ord + Clone + ToString> : Config {
	fn sign(&self, content: &T, signer: Self::AccountId) -> Signature<T>{
		let binding = signer.to_string();
  		let sig = binding.get(0..4).ok_or("Account Index too small")?;
		let content = content.clone();
		content.to_string().push_str(sig);
		Ok(content)
	}
}



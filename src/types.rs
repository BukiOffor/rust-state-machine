use std::ops::AddAssign;

use num::{One, Zero};


pub trait Config {
    type AccountId: Ord;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}


pub type AccountId = String;
pub type Balance = u128;
pub type Nonce = u32;
pub type BlockNumber = u32;
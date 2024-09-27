use std::ops::AddAssign;
use std::fmt::Debug;

use num::{CheckedAdd, CheckedSub, One, Zero};


pub trait Config {
    type AccountId: Ord + Clone + Debug;
    type BlockNumber: Zero + One + AddAssign + Copy + CheckedAdd + CheckedSub + Debug;
    type Nonce: Zero + One + Copy + CheckedAdd + CheckedSub + Debug;
    type Balance: Zero + CheckedSub + CheckedAdd + Copy + AddAssign + One + Debug;

}

pub struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
        type Balance = u128;

    }
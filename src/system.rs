use std::collections::BTreeMap;
use num::traits::{One, Zero};
use core::ops::AddAssign;

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}


#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T>{
    pub fn new() -> Self {
        Self { block_number : T::BlockNumber::zero(), nonce: BTreeMap::new()}
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let actual_nonce: T::Nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        let new_nonce = actual_nonce + T::Nonce::one();
        self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }


    #[test]
    fn init_system() {
        let mut system = super::Pallet::<TestConfig>::new();
        let alice = "alice".to_string();
//        let bob = "bob".to_string();

        system.inc_block_number();
        system.inc_nonce(&alice);
        assert_eq!(system.block_number(), 1);
        assert_eq!(system.nonce.get("alice"), Some(&1));
        assert_eq!(system.nonce.get("bob"), None);
        /* TODO: Create a test which checks the following:
            - Increment the current block number.
            - Increment the nonce of `alice`.

            - Check the block number is what we expect.
            - Check the nonce of `alice` is what we expect.
            - Check the nonce of `bob` is what we expect.
        */
    }
}


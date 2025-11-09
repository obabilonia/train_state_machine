use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

type AccountId = String;
type Nonce = u32;
type BlockNumber = u32;

#[derive(Debug)]
pub struct Pallet {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { block_number : 0, nonce: BTreeMap::new()}
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self.block_number + 1u32;
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let actual_nonce: u32 = *self.nonce.get(who).unwrap_or(&0u32);
        let new_nonce = actual_nonce + 1u32;
        self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        let mut system = super::Pallet::new();
        system.inc_block_number();
        system.inc_nonce(&"alice".to_string());
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


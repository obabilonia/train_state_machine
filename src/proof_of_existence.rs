use core::fmt::Debug;
use std::collections::BTreeMap;
use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
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
    /* TODO: Add a field `claims` which is a `BTreeMap` fom `T::Content` to `T::AccountId`. */
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the Proof of Existence Module.
    pub fn new() -> Self {
        /* TODO: Return a new instance of the `Pallet` struct. */
        Self { claims: BTreeMap::new() }
    }

    /// Get the owner (if any) of a claim.
//    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
//        *self.claims.get(claim).unwrap_or(None)

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
}

#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err("This content has already been claimed.");
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.claims.get(&claim) {
            Some(owner) if *owner == caller => {
                self.claims.remove(&claim);
                Ok(())
            }
            Some(_) => Err("You are not the owner of this claim."),
            None => Err("This claim does not exist."),
        }
    }

}


#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = String;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        /*
            TODO:
            Create an end to end test verifying the basic functionality of this pallet.
                - Check the initial state is as you expect.
                - Check that all functions work successfully.
                - Check that all error conditions error as expected.
        */
        let mut poe = super::Pallet::<TestConfig>::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        let content = "my_content".to_string();
        assert_eq!(poe.get_claim(&content), None);
        assert_eq!(poe.create_claim(alice.clone(), content.clone()), Ok(()));
        assert_eq!(poe.get_claim(&content), Some(&alice));
        assert_eq!(poe.create_claim(bob.clone(), content.clone()), Err("This content has already been claimed."));
        assert_eq!(poe.revoke_claim(bob.clone(), content.clone()), Err("You are not the owner of this claim."));
        assert_eq!(poe.revoke_claim(alice.clone(), content.clone()), Ok(()));
        assert_eq!(poe.get_claim(&content), None);
    }
}
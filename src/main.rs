mod balances;
mod support;
mod system;
use crate::support::Dispatch;


mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
}


// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
   system: system::Pallet<Self>,
   balances: balances::Pallet<Self>,
}


impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}


impl Runtime {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(), 
            balances: balances::Pallet::new()
        }
    }


    // Execute a block of extrinsics. Increments the block number.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {

        self.system.inc_block_number();
        
        if block.header.block_number != self.system.block_number() {
            return Err("block number does not match what is expected")
        }

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

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
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
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}



fn main() {
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    // // start emulating a block
    // runtime.system.inc_block_number();
    // assert_eq!(runtime.system.block_number(), 1);

    // // first transaction
    // runtime.system.inc_nonce(&alice);
    // let _res = runtime.balances.transfer(alice.clone(), bob, 30).map_err(|e| println!("Transaction failed: {}", e));

    // // second transaction
    // runtime.system.inc_nonce(&alice);
    // let _res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| println!("Transaction failed: {}", e));

    // println!("{runtime:#?}");
    //println!("Hello, world!");

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: "alice".to_string(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: bob, amount: 30 }),
            },
            support::Extrinsic {
                caller: "alice".to_string(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie, amount: 20 }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block");

    println!("{runtime:#?}");

}


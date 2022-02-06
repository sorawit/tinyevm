pub mod database;
pub mod runtime;
pub mod state;
pub mod vm;

use ethereum_types::Address;

fn main() {
    let db = database::MemoryDatabase::new();
    let code = hex::decode("6080604052348015600f57600080fd5b506004361060285760003560e01c8063f8a8fd6d14602d575b600080fd5b602a60405190815260200160405180910390f3fea26469706673582212208664c2423e381d1cdf745548b3ccdb3ef3c938966a524fa991bf97373fe040a764736f6c634300080b0033").unwrap();
    let mut vm = vm::VM::new(db, code.as_slice());
    let data = hex::decode("f8a8fd6e").unwrap();
    vm.run(Address::zero(), &data);
    let data = hex::decode("f8a8fd6d").unwrap();
    vm.run(Address::zero(), &data);
    println!("Done!");
}

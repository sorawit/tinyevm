pub mod database;
pub mod error;
pub mod mem;
pub mod runtime;
pub mod stack;
pub mod state;
pub mod vm;

use ethereum_types::Address;

fn main() {
    let db = database::MemoryDatabase::new();
    let code = hex::decode("6080604052348015600f57600080fd5b5060043610603c5760003560e01c80630a8e8e0114604157806366e41cb7146077578063f8a8fd6d14607d575b600080fd5b60408051600160208083019190915282518083038201815291830190925280519101205b60405190815260200160405180910390f35b60656083565b602a6065565b60405162461bcd60e51b815260206004820152600360248201526209cc2d60eb1b604482015260009060640160405180910390fdfea2646970667358221220eac38cfa57a6409ecc85248255995232d566f9cf9a58d0d1d50c48dcb6284ee564736f6c634300080b0033").unwrap();
    // 35452504136398347791722757567016336830725519306142400114911765331455690932224
    let mut vm = vm::VM::new(db, code.as_slice());
    let data = hex::decode("66e41cb7").unwrap();
    vm.run(Address::zero(), &data);
    let data = hex::decode("f8a8fd6d").unwrap();
    vm.run(Address::zero(), &data);
    let data = hex::decode("f8a8fd6e").unwrap();
    vm.run(Address::zero(), &data);
    let data = hex::decode("0a8e8e01").unwrap();
    vm.run(Address::zero(), &data);
    println!("Done!");
}

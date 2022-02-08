mod database;
mod mem;
mod runtime;
mod stack;
mod state;
mod types;
mod vm;

use ethereum_types::Address;

fn main() {
    let db = database::MemoryDatabase::new();
    let code = hex::decode("608060405234801561001057600080fd5b50600436106100575760003560e01c80630a8e8e011461005c5780630c55699c1461009257806366e41cb71461009b578063f6315ea4146100a3578063f8a8fd6d146100ad575b600080fd5b60408051600160208083019190915282518083038201815291830190925280519101205b60405190815260200160405180910390f35b61008060005481565b6100806100b4565b6100ab6100e9565b005b602a610080565b60405162461bcd60e51b815260206004820152600360248201526209cc2d60eb1b604482015260009060640160405180910390fd5b6000805490806100f8836100ff565b9190505550565b600060001982141561012157634e487b7160e01b600052601160045260246000fd5b506001019056fea26469706673582212201d783797175d5d6489f0e54afbd25817ef774287e562f5d1f00e76b18e668bb364736f6c634300080b0033").unwrap();
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
    let data = hex::decode("f6315ea4").unwrap();
    vm.run(Address::zero(), &data);
    let data = hex::decode("0c55699c").unwrap();
    vm.run(Address::zero(), &data);
    println!("Done!");
}

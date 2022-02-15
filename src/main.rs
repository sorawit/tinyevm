mod db;
mod io;
mod mem;
mod runtime;
mod stack;
mod state;
mod types;
mod vm;

use ethereum_types::Address;
use tempdir::TempDir;

fn main() {
    let mut fio = io::FileIO::new(std::path::Path::new("./data.json"));
    use io::IO;
    println!("ez {:?}", fio.get_code());
    println!("ez {:?}", fio.get_next_env());
    println!("ez {:?}", fio.get_next_env());
    let dir = TempDir::new("maintest").unwrap();
    let db = db::LevelDB::new(&dir.path());
    let code = hex::decode("608060405234801561001057600080fd5b50600436106100575760003560e01c80630a8e8e011461005c5780630c55699c1461009257806366e41cb71461009b578063980cd0fc146100a3578063f8a8fd6d146100b8575b600080fd5b60408051600160208083019190915282518083038201815291830190925280519101205b60405190815260200160405180910390f35b61008060005481565b6100806100bf565b6100b66100b136600461014d565b6100f4565b005b602a610080565b60405162461bcd60e51b815260206004820152600360248201526209cc2d60eb1b604482015260009060640160405180910390fd5b80600080828254610105919061017c565b90915550506000547f7afbe4f1c55b5f72ea356f5b4d5615831867af31454a5ca5557f315e6d11a369610139826002610194565b60405190815260200160405180910390a250565b60006020828403121561015f57600080fd5b5035919050565b634e487b7160e01b600052601160045260246000fd5b6000821982111561018f5761018f610166565b500190565b60008160001904831182151516156101ae576101ae610166565b50029056fea2646970667358221220c41b85ba8877c9796fab15e49e19a8ed9fe5555caf085bc32e68848d65fa9e4564736f6c634300080b0033").unwrap();
    // 35452504136398347791722757567016336830725519306142400114911765331455690932224
    let mut vm = vm::VM::new(db, code.as_slice());
    let mut env = types::Env {
        caller: Address::zero(),
        timestamp: 0.into(),
        number: 0.into(),
        chainid: 1.into(),
        calldata: vec![],
    };
    env.calldata = hex::decode("66e41cb7").unwrap();
    println!("{:?}", vm.run(&env));
    env.calldata = hex::decode("f8a8fd6d").unwrap();
    println!("{:?}", vm.run(&env));
    env.calldata = hex::decode("f8a8fd6e").unwrap();
    println!("{:?}", vm.run(&env));
    env.calldata = hex::decode("0a8e8e01").unwrap();
    println!("{:?}", vm.run(&env));
    env.calldata = hex::decode("980cd0fc000000000000000000000000000000000000000000000000000000000000002a").unwrap();
    println!("{:?}", vm.run(&env));
    env.calldata = hex::decode("0c55699c").unwrap();
    println!("{:?}", vm.call(&env));
    println!("Done!");
}

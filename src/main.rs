pub mod database;
pub mod state;

fn main() {
    let db = database::MemoryDatabase::new();
    let _ = state::State::new(db);
    println!("Hello, world!");
}

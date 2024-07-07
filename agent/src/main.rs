use ralo_core::add;

pub mod agent;
pub mod trade;

#[tokio::main]
async fn main() {
    add(2, 2);

    println!("Hello, world!");
}

fn main() {
    let subcommand = std::env::args().nth(1).expect("subcommand is not given");

    println!("Hello, world!");
}

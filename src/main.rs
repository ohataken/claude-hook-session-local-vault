mod dir;

fn main() {
    let subcommand = std::env::args().nth(1).expect("subcommand is not given");

    if subcommand == "dir" {
        dir::run();
    } else {
        panic!("unknown subcommand: {subcommand}");
    }
}

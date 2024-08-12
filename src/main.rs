use rls::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args[0]);
    let config = Config::build(&args).unwrap();
    Config::run(config).unwrap();
}

use std::env;
use std::fs;
use std::path::PathBuf;

pub struct Config {
    pub path: PathBuf,
    pub contents: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let args_len = args.len();
        let path = env::current_dir().unwrap();
        let mut contents = String::new();

        if args_len > 2 {
            for item in args[2..].into_iter() {
                contents = contents + " " + item;
            }
        } else {
            contents = String::from("NONE");
        };

        Ok(Config { path, contents })
    }

    // TODO: Implament command flags
    pub fn run(config: Config) -> Result<(), ()> {
        if config.contents == "NONE" {
            let dir_list = fs::read_dir(config.path).unwrap();
            for item in dir_list {
                let new_str = item.unwrap().file_name().into_string().unwrap();
                println!("{new_str}");
            }
        } else {
            println!("FAIL");
        }
        Ok(())
    }
}

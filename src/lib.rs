
use std::fs;
use std::error::Error;

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("with text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn new(args:&[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
// fn parse_config(args:&[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

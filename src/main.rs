use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    })

    // let query = &args[1];
    // let file_path = &args[2];

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // let contents = fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    // println!("with text:\n{contents}");
    // println!("{:?}", args);

    if let Err(e) = run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }

}

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;

//     println!("with text:\n{contents}");
// }

// struct Config {
//     query: String,
//     file_path: String,
// }
// impl Config {
//     fn new(args:&[String]) -> Result<Config, &str> {
//         if args.len() < 3 {
//             return Err("Not enough arguments");
//         }
        
//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config { query, file_path })
//     }
// }
// fn parse_config(args:&[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

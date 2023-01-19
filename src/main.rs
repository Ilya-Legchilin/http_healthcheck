use std::{env, process};
use std::{thread, time::Duration};
use reqwest::StatusCode;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1)
    });
    loop {
        run(config.clone()).await?;
    }
}

#[derive(Clone)]
pub struct Config {
    pub interval: u64,
    pub url: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let interval = match args[1].clone().parse() {
            Err(_) => return Err("failure parsing interval"),
            Ok(value) => value,
        };
        let url = args[2].clone();
        match Url::parse(&url) {
            Err(_) => return Err("URL parsing error"),
            _ => (),
        }
        Ok(Config {
            interval,
            url,
        })
    }
}

async fn run(config: Config) -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::get(config.url.clone()).await?;
    let status = match res.status() {
        StatusCode::OK => format!("OK(200)"),
        _ => format!("ERR({})", res.status().as_u16())
    };
    println!("Checking '{}'. Result: {}", config.url, status);
    let response = format!("Checking '{}'. Result: {}", config.url, res.status());
    thread::sleep(Duration::from_secs(config.interval));
    Ok(response)
}
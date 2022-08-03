use clap::Parser;
use std::{fs, process::exit};
mod servers;
mod utils;
use servers::{use_build_in_config, use_file_config};
use utils::send_post;

use crate::servers::Config;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of the file to upload
    #[clap(min_values = 1, max_values = 99)]
    path: Vec<String>,

    /// Select server
    #[clap(short = 's', long)]
    server: Option<String>,

    /// show all avaliable servers
    #[clap(short, long)]
    list: bool,

    /// Use a config file
    #[clap(short, long)]
    config: Option<String>,

    /// Turn debugging information on
    #[clap(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut server_name = String::from("smms"); // default to-upload server name
    let servers: Vec<Config>; // servers from config

    if let Some(config_path) = args.config {
        servers = use_file_config(config_path); // load the config file
    } else {
        servers = use_build_in_config(); // use build-in
    }

    if let Some(s) = args.server {
        server_name = s;
    }

    // for --list
    if args.list {
        println!(" Servers avaliable:\n");
        for s in servers {
            println!(" {}", s.name);
        }
        exit(0);
    }

    // check if input files is set
    if args.path.len() == 0 {
        // eprintln!(" \x1b[1;40mUpload Fail\x1b[0m");
        eprintln!("Please enter one or more paths for uploading!");
        eprintln!("  Use flag `--help` for help");
        exit(1);
    }

    // check if the server is set in config file
    let mut config: Option<Config> = None;
    for s in servers {
        if s.name == server_name {
            config = Some(s);
        }
    }
    let config = config.unwrap();

    let mut success_urls = Vec::new();
    // load input files
    for path in args.path {
        match upload_one(&path, &config, args.debug).await {
            Ok(url) => success_urls.push(url),
            Err(err) => {
                println!(" \x1b[1;40mUpload Fail\x1b[0m");
                print_urls(success_urls);
                println!("Fail to upload `{}`, Reason: {}", path, err);
                exit(1)
            }
        }
    }

    println!(" \x1b[1;40mUpload Success\x1b[0m");
    print_urls(success_urls);
    exit(0);
}

async fn upload_one(path: &String, config: &Config, debug: bool) -> Result<String, String> {
    let path = path.clone();
    match fs::metadata(&path) {
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(format!("`{}` is not a file", path));
            }
        }
        Err(e) => {
            return Err(format!("Fail to read file `{}`.\nReason: {}", path, e));
        }
    }
    // upload
    if debug {
        println!("{:#?}\n", config);
    }
    send_post(config, path, debug).await
}

fn print_urls(urls: Vec<String>) {
    for url in urls.iter() {
        println!("{}", url)
    }
}

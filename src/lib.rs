use clap::{ Arg, App, AppSettings };
use std::fs;
use reqwest::{ self, Client, Response, };
use std::process;

// Structure to keep user configs
pub struct Configs {
    pub wordlist: String,
    pub url: String,
    pub threads: usize,
    pub wait: u64,
}

impl Configs {
    fn new() -> Configs {
        Configs{ wordlist: String::new(), url: String::new(), threads: 40, wait: 100 }
    }
}

pub struct Brutef {
    pub client: Client,
}


impl Brutef {
    pub fn new() -> Brutef {
        Brutef{ client: Client::builder().redirect(reqwest::redirect::Policy::none()).connect_timeout(std::time::Duration::from_secs(10)).build().unwrap() }
    }

    #[tokio::main]
    pub async fn request(&self, url: String) -> Result<Response, reqwest::Error> {
        return self.client.get(&url).send().await
    }
}

// Parser all user commands and return structure of type 'Configs'
pub fn parse_args() -> Configs {
    let app = App::new("bruteforus")
            .setting(AppSettings::ColoredHelp)
            .setting(AppSettings::DeriveDisplayOrder)
            .setting(AppSettings::UnifiedHelpMessage)
            .setting(AppSettings::NextLineHelp)
            .setting(AppSettings::HidePossibleValuesInHelp)
            .max_term_width(90)
            .version("0.1.0")
            .help("Used to bruteforce urls.");

    let wordlist_arg = Arg::with_name("wordlist")
        .long("wordlist")
        .short("w")
        .takes_value(true)
        .help("Wordlist to bruteforce with URL")
        .required(true);

    let wait_arg = Arg::with_name("wait")
        .long("wait")
        .takes_value(true)
        .help("MilliSecs to wait after each request")
        .required(false);

    let url_arg = Arg::with_name("url")
        .help("Target URL")
        .required(true);
    
    let thread_arg = Arg::with_name("threads")
        .long("threads")
        .takes_value(true)
        .short("t")
        .help("Number of threads to perform requests")
        .required(false);

    let app = app.arg(wordlist_arg);
    let app = app.arg(url_arg);
    let app = app.arg(thread_arg);
    let app = app.arg(wait_arg);
    
    let matches = app.get_matches();

    let mut user_configs = Configs::new();

    user_configs.wordlist = match matches.value_of("wordlist") {
        Some(r) => { r.to_string() },
        None => { 
            eprintln!("[-] Error! - Please specify wordlist.");
            process::exit(1);
        }
    };

    user_configs.url = match matches.value_of("url") {
        Some(r) => { r.to_string() },
        None => {
            eprintln!("[-] Error! - Please specify url.");
            process::exit(1);
        }
    };
    
    user_configs.threads = match matches.value_of("threads") {
        Some(r) => { 
            match r.trim().parse() {
                Ok(r) => r,
                Err(_er) => {
                    eprintln!("[-] Error - Error in parsing thread's value");
                    process::exit(1);
                }
            } 
        },
        None => {
            40
        }
    };
    
    user_configs.wait = match matches.value_of("wait") {
        Some(r) => { 
            match r.trim().parse() {
                Ok(r) => r,
                Err(_er) => {
                    eprintln!("[-] Error - Error in parsing wait value");
                    process::exit(1);
                }
            } 
        },
        None => {
            100
        }
    };



    if user_configs.url.chars().nth(user_configs.url.chars().count() - 1).unwrap() != '/' {
        user_configs.url = format!("{}/", user_configs.url);
    }

    println!("\n++==================================================================================================++");
    println!("=>--------------------------------------------------------------------------------------------------<=");
    println!("\n\tURL\t\t=>\t\t{}\n\tWordlist\t=>\t\t{}\n\tThread\t\t=>\t\t{}", user_configs.url, user_configs.wordlist, user_configs.threads);
    println!("\n=>--------------------------------------------------------------------------------------------------<=");
    println!("++==================================================================================================++\n");

    user_configs
}

// Read file and return String of file contents
pub fn get_file_contents(wordlist_filename: &str) -> String {
    let file_contents: String = match fs::read_to_string(wordlist_filename) {
        Ok(w) => w,
        Err(_er) => {
            eprintln!("Error in opening file");
            std::process::exit(1);
        },
    };

    file_contents
}

/*
Possibility of sending custom headers and parameters. Added to TODO.

let params = [("foo", "bar"), ("baz", "quux")];
*/

// Make GET request on provided URL and return Result object.
#[tokio::main]
pub async fn request(target_url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::builder().redirect(reqwest::redirect::Policy::none()).connect_timeout(std::time::Duration::from_secs(10)).build().unwrap();
    
    // '&*' in '&*target_url' is required to convert type Box<String> to &str
    let req = client.get(&*target_url);
    req.send().await
}

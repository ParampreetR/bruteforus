use clap::{ Arg, App, AppSettings };

fn build() -> App<'static, 'static> {
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
    app
}
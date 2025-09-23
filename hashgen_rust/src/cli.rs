// src/cli.rs
use clap::Parser;

/// command-line options for hashgen
#[derive(Parser, Debug)]
#[command(
    name = "hashgen",
    about = "hashgen (Rust)",
    disable_help_flag = true,       // use custom help
    disable_version_flag = true,    // use custom -version
    allow_hyphen_values = true,     // let Clap accept -help
    allow_negative_numbers = true
)]
pub struct Opts {
    // hash function to use
    #[arg(short = 'm', long = "m", value_name = "MODE", default_value = "")]
    pub mode: String,

    // input wordlist (default: stdin)
    #[arg(short = 'w', long = "w", default_value = "")]
    pub input: String,

    // output file (default: stdout)
    #[arg(short = 'o', long = "o", default_value = "")]
    pub output: String,

    // output hash:plain
    #[arg(long = "hashplain", default_value_t = false)]
    pub hashplain: bool,

    // benchmark mode (disables output)
    #[arg(short = 'b', long = "b", default_value_t = false)]
    pub bench: bool,

    // CPU threads (default = 0 (all)
    #[arg(short = 't', long = "t", default_value_t = 0)]
    pub threads: i32,

    // bcrypt cost (4 - 31) for bcrypt / wpbcrypt
    #[arg(long = "cost", visible_alias = "cost", default_value_t = 10)]
    pub cost: u32,

    // show custom version info
    #[arg(long = "version", visible_alias = "version", default_value_t = false)]
    pub ver: bool,

    // show dev info (Easter egg)
    #[arg(long = "cyclone", visible_alias = "cyclone", default_value_t = false)]
    pub cyclone: bool,

    // show custom help screen
    #[arg(long = "help", visible_alias = "help", default_value_t = false)]
    pub show_help: bool,
}

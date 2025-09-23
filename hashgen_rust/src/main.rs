// src/main.rs

mod algo;
mod cli;
mod consts;
mod io;

use anyhow::Result;
use base64::Engine as _;
use clap::Parser;

use crate::cli::Opts;
use crate::consts::{help_text, version};
use crate::io::start_proc;

/*
Program:    hashgen (Rust) - a complete (experimental) rewrite of hashgen (Go)
License:    GPLv2
Author:     cyclone
Github:     https://github.com/cyclone-github/hashgen-testing/tree/main/hashgen_rust

I am not a Rust software developer, so porting hashgen (Go) to Rust has been challenging with lots of trial and error.
Consider this an experimental POC and not production quality
hashgen (Go) will remain my primary implementation of hashgen, and I do not expect hashgen (Rust) to be maintained.

changelog
v2023-10-30.1615
    initial github release
v2024-05-01.1100
    add support for non-UTF8 char, read input as bytes, add support for hashcat modes (-m)
v1.2.0-rust; 2025-09-23
    complete rewrite of hashgen (Go) v1.2.0-dev in Rust

TODO -- probably won't happen any time soon as my focus is on hashgen (Go)
*/

fn main() -> Result<()> {
    let opts = Opts::parse();

    if opts.ver {
        version();
        return Ok(());
    }
    if opts.cyclone {
        let s = base64::engine::general_purpose::STANDARD
            .decode("Q29kZWQgYnkgY3ljbG9uZSA7KQo=")
            .map(|v| String::from_utf8_lossy(&v).to_string())
            .unwrap_or_else(|_| "--> Cannot decode base64 string. <--".to_string());
        eprintln!("{s}");
        return Ok(());
    }
    if opts.show_help {
        help_text();
        return Ok(());
    }
    if opts.mode.is_empty() {
        eprintln!("--> missing '-m algo' <--");
        help_text();
        std::process::exit(1);
    }
    if opts.bench && !opts.output.is_empty() {
        eprintln!("Error: -o cannot be used with -b (benchmark mode)");
        std::process::exit(1);
    }

    let needs_cost = matches!(opts.mode.as_str(), "bcrypt" | "3200" | "wpbcrypt");
    if !needs_cost && opts.cost != 10 {
        eprintln!("Error: -cost flag is only allowed for bcrypt modes");
        std::process::exit(1);
    }
    if needs_cost && (opts.cost < 4 || opts.cost > 31) {
        eprintln!("Invalid bcrypt cost: must be between 4 and 31");
        std::process::exit(1);
    }

    // threads
    let max_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let used = if opts.threads <= 0 {
        max_threads
    } else {
        opts.threads.min(max_threads as i32) as usize
    };
    rayon::ThreadPoolBuilder::new()
        .num_threads(used)
        .build_global()
        .ok();

    start_proc(
        &opts.mode,
        &opts.input,
        &opts.output,
        opts.hashplain,
        used,
        opts.cost,
        opts.bench,
    )
}

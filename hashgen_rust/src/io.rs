// src/io.rs

use anyhow::{Context, Result};
use crossbeam_channel as xch;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use crate::algo::{check_for_hex, hash_line};
pub fn start_proc(
    mode: &str,
    input: &str,
    output: &str,
    hashplain: bool,
    threads: usize,
    cost: u32,
    bench: bool,
) -> Result<()> {
    // buffer
    let mut read_buf = threads + 16 * 32 * 1024;
    if matches!(
        mode,
        "phpass" | "phpbb3" | "400" | "md5crypt" | "500" | "sha256crypt" | "7400" | "sha512crypt"| "1800") {
        read_buf = threads + 16 * 32;
    }
    if matches!(mode, "bcrypt" | "3200" | "wpbcrypt") {
        read_buf = threads.saturating_div(cost as usize).max(1) + 64;
    }
    if matches!(mode, "argon2id" | "34000" | "yescrypt") {
    read_buf = threads + 16;
    }
    let write_buf = 2 * read_buf;

    // IO setup
    let mut rdr: Box<dyn Read> = if input.is_empty() {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input).with_context(|| format!("open input: {input}"))?)
    };
    let wtr: Box<dyn Write> = if bench {
        Box::new(io::sink())
    } else if output.is_empty() {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(output).with_context(|| format!("create output: {output}"))?)
    };

    eprintln!("Starting...");
    if input.is_empty() {
        eprintln!("Reading from stdin...");
    } else {
        eprintln!("Processing file: {input}");
    }
    eprintln!("Hash function: {mode}");
    eprintln!("CPU Threads: {threads}");

    let start = Instant::now();

    // channels
    let (tx_read, rx_read) = xch::unbounded::<(i64, Vec<u8>)>();
    let (tx_write, rx_write) = xch::unbounded::<(i64, Vec<u8>)>();

    // reader on main thread
    {
        let mut buf = vec![0u8; read_buf];
        let mut remainder: Vec<u8> = Vec::new();
        let mut idx = 0i64;

        loop {
            let n = match rdr.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            let mut chunk = Vec::with_capacity(remainder.len() + n);
            chunk.extend_from_slice(&remainder);
            chunk.extend_from_slice(&buf[..n]);

            if let Some(last_nl) = chunk.iter().rposition(|&b| b == b'\n') {
                let front = chunk[..=last_nl].to_vec();
                remainder = chunk[last_nl + 1..].to_vec();
                if tx_read.send((idx, front)).is_err() {
                    break;
                }
                idx += 1;
            } else {
                remainder = chunk;
            }
        }
        if !remainder.is_empty() {
            let _ = tx_read.send((idx, remainder));
        }
        drop(tx_read); // close producer
    }

    // proc worker (parallel over incoming chunks)
    let lines_hashed = Arc::new(AtomicI64::new(0));
    let hex_errs = Arc::new(AtomicI64::new(0));

    {
        let tx_write = tx_write.clone();
        let mode = mode.to_string();
        let lines_hashed_cl = Arc::clone(&lines_hashed);
        let hex_errs_cl = Arc::clone(&hex_errs);

        std::thread::spawn(move || {
            rx_read.into_iter().par_bridge().for_each(|(idx, data)| {
                let mut outbuf: Vec<u8> = Vec::with_capacity(write_buf);
                for line in data.split(|&b| b == b'\n') {
                    if line.is_empty() {
                        continue;
                    }
                    let (decoded, disp, herr) = check_for_hex(line);
                    if herr != 0 {
                        hex_errs_cl.fetch_add(herr as i64, Ordering::Relaxed);
                    }
                    if let Ok(Some(hashv)) = hash_line(&mode, &decoded, cost) {
                        if hashv.is_empty() {
                            continue;
                        }
                        outbuf.extend_from_slice(&hashv);
                        if hashplain {
                            outbuf.push(b':');
                            outbuf.extend_from_slice(&disp);
                        }
                        outbuf.push(b'\n');
                        lines_hashed_cl.fetch_add(1, Ordering::Relaxed);
                    }
                }
                let _ = tx_write.send((idx, outbuf));
            });
        });
    }
    drop(tx_write);

    // writer on main thread
    let mut pending: BTreeMap<i64, Vec<u8>> = BTreeMap::new();
    let mut next = 0i64;
    let mut bw = BufWriter::with_capacity(write_buf, wtr);

    for (idx, data) in rx_write.iter() {
        pending.insert(idx, data);
        while let Some(buf) = pending.remove(&next) {
            let _ = bw.write_all(&buf);
            let _ = bw.flush();
            next += 1;
        }
    }
    while let Some(buf) = pending.remove(&next) {
        let _ = bw.write_all(&buf);
        let _ = bw.flush();
        next += 1;
    }
    let _ = bw.flush();

    // stats
    let dt = start.elapsed().as_secs_f64();
    let lines = lines_hashed.load(Ordering::Relaxed);
    let lps = if dt > 0.0 { (lines as f64) / dt } else { 0.0 };
    let (scaled, unit) = if lps >= 1e12 {
        (lps / 1e12, "T")
    } else if lps >= 1e9 {
        (lps / 1e9, "B")
    } else if lps >= 1e6 {
        (lps / 1e6, "M")
    } else if lps >= 1e3 {
        (lps / 1e3, "K")
    } else {
        (lps, "")
    };

    let herr = hex_errs.load(Ordering::Relaxed);
    if herr > 0 {
        eprintln!("HEX decode errors: {herr}");
    }
    if unit.is_empty() {
        eprintln!(
            "Finished processing {} lines in {:.3} sec ({:.3} lines/sec)",
            lines, dt, scaled
        );
    } else {
        eprintln!(
            "Finished processing {} lines in {:.3} sec ({:.3} {} lines/sec)",
            lines, dt, scaled, unit
        );
    }
    Ok(())
}

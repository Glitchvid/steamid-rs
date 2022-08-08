//! Sample application that parses an input into a SteamId and manipulates it.

extern crate steamid;

use std::env;

use steamid::{IdFormat, SteamId};

fn main() {
    // Gather our CLI arguments
    let args: Vec<String> = env::args().collect();

    // Dumb check, make sure they even tried providing a SteamID
    (args.len() >= 2).then(|| ()).unwrap_or_else(|| {
        println!("No IDs provided!");
        std::process::exit(-1);
    });

    // Process all of our passed strings
    for input in args.iter().skip(1) {
        match input.parse::<SteamId>() {
            Ok(v) => {
                println!("steamID64:\t{}", IdFormat::SteamId64(&v));
                println!("steamID:  \t{}", IdFormat::SteamId2(&v));
                println!("steamID3: \t{}", IdFormat::SteamId3(&v));
            }
            Err(e) => {
                println!("Unable to parse \"{}\" reason: '{}'", input, e);
                continue;
            }
        }
    }
    println!();
}

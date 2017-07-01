//   Copyright 2017 Christopher Vittal <christopher.vittal@gmail.com>
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.
extern crate clap;
use clap::{App, Arg};

use std::fs::File;
use std::io;
use std::io::prelude::*;

// TODO : Make these not hardcoded?
const MIN_BRIGHTNESS: u16 = 1;
const MAX_BRIGHTNESS: u16 = 7500;


static INPUT_HELP: &'static str = "Sets brightness by either absolute or percentage,
the valid range is between 1 to 7500 or 0% to 100%.";

static BRIGHT_PATH: &'static str = "/sys/class/backlight/intel_backlight/brightness";

fn main() {

    let matches = App::new("Candlelight")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Chris Vittal <christopher.vittal@gmail.com>")
        .about("A tiny utility to get/set the brightness of my laptop")
        .arg(Arg::with_name("INPUT").group("input").help(INPUT_HELP))
        .arg(
            Arg::with_name("preview")
                .short("p")
                .long("preview")
                .requires("input")
                .help("Previews change. Does not change any settings"),
        )
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("current-settings")
                .conflicts_with("input")
                .help("Displays current settings"),
        )
        .arg(
            Arg::with_name("minimum")
                .short("m")
                .conflicts_with("INPUT")
                .conflicts_with("maximum")
                .group("input")
                .help("Sets brightness to minimum value"),
        )
        .arg(
            Arg::with_name("maximum")
                .short("M")
                .conflicts_with("INPUT")
                .conflicts_with("minimum")
                .group("input")
                .help("Sets brightness to maximum value"),
        )
        .get_matches();
    //println!("matches:\n  {:?}\n\n", matches);

    let is_query = matches.is_present("query") || !matches.is_present("input");
    let dry_run = matches.is_present("preview");
    let old_brightness: u16;

    if is_query || dry_run {
        old_brightness = match get_brightness() {
            Ok(b) => b,
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        };
        if is_query {
            println!(
                "brightness:{:8}\t{:8.*}%",
                old_brightness,
                3,
                100. * (old_brightness as f64 / MAX_BRIGHTNESS as f64)
            );
            return;
        }
    } else {
        old_brightness = 0;
    }


    let target_brightness = if matches.is_present("INPUT") {
        let tmp = match matches.value_of("INPUT") {
            Some(v) => parse_input_value(v),
            None => panic!("Shouldn't get here"),
        };
        tmp.unwrap_or_else(|e| e.exit())
    } else if matches.is_present("maximum") {
        MAX_BRIGHTNESS
    } else if matches.is_present("minimum") {
        MIN_BRIGHTNESS
    } else {
        panic!("Argment issue, shouldn't get here, possible clap bug?")
    };

    match write_brightness(target_brightness) {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    }
    // It is possible that there was an error, but the file was still written, so we
    // continue even if the above was an error.
    if !dry_run {
        return;
    }
    assert!(old_brightness > 0);
    std::thread::sleep(std::time::Duration::new(3, 0));
    match write_brightness(old_brightness) {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    }
}

fn get_brightness() -> io::Result<u16> {
    let mut file = File::open(BRIGHT_PATH)?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;
    let mut bright = 0;
    for &b in &buf {
        if b < b'0' || b > b'9' {
            break;
        }
        bright = 10 * bright + (b - b'0') as u16;
    }
    Ok(bright)
}

fn write_brightness(target: u16) -> io::Result<()> {
    let mut file = File::create(BRIGHT_PATH)?;
    write!(&mut file, "{}", target)?;
    file.flush()?;
    Ok(())
}

fn parse_input_value(val: &str) -> clap::Result<u16> {
    let ret_val = if val.ends_with('%') {
        let new_val = val.trim_right_matches('%');
        match new_val.parse::<f64>() {
            Ok(v) => Ok((v * MAX_BRIGHTNESS as f64 / 100.0) as u16),
            Err(_) => Err(clap::Error::value_validation_auto(
                format!("The argument '{}' isn't a valid value", val),
            )),
        }
    } else {
        match val.parse::<u16>() {
            Ok(v) => Ok(v),
            Err(_) => Err(clap::Error::value_validation_auto(
                format!("The argument '{}' isn't a valid value", val),
            )),
        }
    };
    if let Ok(v) = ret_val {
        if v == 0 {
            Ok(1)
        } else if v > MAX_BRIGHTNESS || v < MIN_BRIGHTNESS {
            let too_high_low = if v > MAX_BRIGHTNESS {
                "too high: max value is 7500 or 100%"
            } else {
                "too low: min value is 1 or 0%"
            };
            Err(clap::Error::value_validation_auto(format!(
                "The argument '{}' isn't a valid value ({})",
                val,
                too_high_low
            )))
        } else {
            Ok(v)
        }
    } else {
        ret_val
    }
}

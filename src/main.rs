#[macro_use]
extern crate clap;
extern crate futures;
extern crate tokio_core;
extern crate tokio_process;

use std::process::{Command, exit};

use clap::App;
use tokio_core::reactor::Core;
use tokio_process::CommandExt;

fn main() {
    let yaml    = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let tries              = matches.value_of("tries").unwrap_or("10");
    let num_tries          = tries.parse::<usize>().ok().unwrap_or(10);
    let program: Vec<&str> = matches.values_of("program").unwrap_or_default().collect();

    if program.len() <= 0 {
        println!("program arguments must be > 0");
        println!("{}", matches.usage());
        exit(1);
    }

    if num_tries <= 0 {
        println!("-t, --tries must be > 0");
        exit(1);
    }

    let mut core     = Core::new().unwrap();
    let mut commands = Vec::new();
    for _ in 0..num_tries {
        commands.push(Command::new(&program[0])
                      .args(&program[1..])
                      .output_async(&core.handle()));
    }

    let first_one_wins = futures::future::select_all(commands);
    let done           = core.run(first_one_wins).expect("failed to collect output");

    println!("{}", std::str::from_utf8(&done.0.stdout).unwrap_or("Could not convert stdout to utf-8"));
}

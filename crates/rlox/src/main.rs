mod errors;
mod tokens;

use clap::{Arg, Command};
use std::fs::File;
use std::io;
use std::io::{Read, Write};

fn main() {
    let cmd = Command::new("rlox")
        .arg(
            Arg::new("source")
                .value_name("FILE")
                .help("Source code to run"),
        )
        .get_matches();

    match cmd.get_one::<String>("source") {
        Some(source_file_path) => run_file(source_file_path),
        None => run_prompt(),
    }
    .expect("We encountered an error. We are sorry.");
}

fn run_file(source_file_path: &str) -> Result<(), io::Error> {
    let mut f = match File::open(source_file_path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    run(&contents)?;

    Ok(())
}

fn run_prompt() -> Result<(), io::Error> {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to get input");
        run(response.trim())?;
    }
}

fn run(source: &str) -> Result<(), io::Error> {
    println!("{}", source);
    Ok(())
}

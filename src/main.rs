extern crate clap;
use clap::{Arg, App};
use std::process::{Command};

fn main() {
    let matches = App::new("My Super Pipe")
        .version("1.0")
        .author("Kevin BOURMIER")
        .about("Pipe operator")
        .arg(Arg::with_name("input")
            .short("in")
            .long("input")
            .help("input operation")
            .takes_value(true)
            .required(true))

        .arg(Arg::with_name("output")
            .short("out")
            .long("output")
            .help("output operation")
            .takes_value(true)
            .required(true))

        .get_matches();

    let input_operator = matches.value_of("input").unwrap_or("default");
    println!("Value for input: {}", input_operator);

    let output_operator = matches.value_of("output").unwrap_or("default");
    println!("Value for output: {}", output_operator);


    let new_input = Command::new(input_operator.to_string())
        .output()
        .expect("Error Input");

    let get_string_input = String::from_utf8_lossy(&new_input.stdout).to_string();

    let new_output = Command::new(output_operator.to_string())
        .arg(get_string_input)
        .output()
        .expect("Error Output");

    println!("{}", String::from_utf8_lossy(&new_output.stdout).to_string());
}

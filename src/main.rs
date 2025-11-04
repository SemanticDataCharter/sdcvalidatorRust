//! Command-line interface for sdcvalidator

use std::process;

fn main() {
    println!("sdcvalidatorRust - Command Line Interface");
    println!("Status: Planning phase (Q2 2026)");
    println!();
    println!("This project is currently in development.");
    println!("For production use, please see:");
    println!("  - sdcvalidator (Python): https://github.com/SemanticDataCharter/sdcvalidator");
    println!("  - sdcvalidatorJS: https://github.com/SemanticDataCharter/sdcvalidatorJS");
    println!();
    println!("Planned usage:");
    println!("  sdcvalidator validate --schema schema.xsd --input data.xml");
    println!("  sdcvalidator validate --schema schema.xsd --input data.xml --recovery");

    process::exit(0);
}

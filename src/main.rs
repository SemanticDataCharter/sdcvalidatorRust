//! Command-line interface for sdc4-validator

use std::process;

fn main() {
    println!("sdc4-validator - Command Line Interface");
    println!("Status: Planning phase (Q2 2026)");
    println!();
    println!("This project is currently in development.");
    println!("For production use, please see:");
    println!("  - sdcvalidator (Python): https://github.com/SemanticDataCharter/sdcvalidator");
    println!("  - @semanticdatacharter/validator (JavaScript/TypeScript): https://github.com/SemanticDataCharter/sdcvalidatorJS");
    println!();
    println!("Planned usage:");
    println!("  sdc4-validate --schema schema.xsd --input data.xml");
    println!("  sdc4-validate --schema schema.xsd --input data.xml --recovery");

    process::exit(0);
}

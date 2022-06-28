use std::process;

use todo_rust::Config;

fn main() {
    let config = Config::new();

    config.run().unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });
}

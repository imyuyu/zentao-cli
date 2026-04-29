fn main() {
    if let Err(e) = zentao_cli::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

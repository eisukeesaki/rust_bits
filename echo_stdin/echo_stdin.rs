fn main() {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(e) => println!("Failed to read from stdin {}", e),
    }
    println!("{}", input);
}


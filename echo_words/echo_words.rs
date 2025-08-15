fn main() {
    println!("Welcome to echo words. Enter some words separated by a space.");

    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => {},
        Err(e) => eprintln!("{}", e)
    }

    for word in line.split_whitespace() {
        print!("{} ", word);
    }
    println!();
}


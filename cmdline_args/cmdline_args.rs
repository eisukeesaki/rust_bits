fn main() {
   let args: Vec<String> = std::env::args().collect();

   let ip_addr = &args[1];
   let port =  &args[2];
   let socket = format!("{}:{}", ip_addr, port);

   println!("{}", socket);
}


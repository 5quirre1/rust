fn main() {
    let name = if cfg!(windows) {
        std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
    } else if cfg!(unix) {
        std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
    } else {
        "unknown".to_string()
    };
    
    println!("hi {}", name);
}

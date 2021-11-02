fn main() {
    loop {
        println!("Hello, world!");
        eprintln!("Hello, world! on the stderr");
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

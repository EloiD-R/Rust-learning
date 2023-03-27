fn main() {
    let setup_message: &str = setup("");
    let mut x: i32 = 0;
    loop {
        x += 1; 
        println!("{setup_message}");
        print!("x : {x}");
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn setup(_to_use_return: &str) -> &str {
    const SETUP_MESSAGE: &str = "+-----------------+\n|bingo ?          |\n|Yes but in RUST. |\n+-----------------+";
    return SETUP_MESSAGE;
}
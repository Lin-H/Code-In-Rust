use std::io;

fn main() {
    let mut input = String::new();
    println!("type a string");
    io::stdin().read_line(&mut input).unwrap();
    println!("You type : {}", input);
    unsafe {
        input.as_mut_vec().reverse();
    }
    println!("reverse the string, you get {}", input.trim());
}

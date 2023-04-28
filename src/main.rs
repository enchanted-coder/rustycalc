
fn main() {
    
    use std::io;
    print!("Guess the number: \n");
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("You didn't enter anything");

    print!("You guessed the number: {}\n", num)
}
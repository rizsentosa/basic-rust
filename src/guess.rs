use std::io;

pub fn question() {
    println!("guess the number!");
    println!("please input your guess.");

    let applise = "apple"; // immutable
    let mut guess = String::new(); // mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("your guess {guess} {applise}");
}

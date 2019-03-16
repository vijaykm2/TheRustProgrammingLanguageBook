fn main() {
    println!("Chapter 2 Guessing game");
    println!("Guess the number!!");
    println!("Please input your guess!!");
    let mut guess =String::new();
    std::io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("you guessed: {}", guess);
}

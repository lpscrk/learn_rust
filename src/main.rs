use std::io;

fn  main() {
    println!("Guessed number !\n");

    println!("Please input your guess.\n");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line\n");

    println!("You guessed: {}\n", guess);
}
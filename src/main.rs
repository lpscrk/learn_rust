use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn  main() {
    println!("Guessed number !\n");
    
    let secret_number = rand::thread_rng().gen_range(1..=100); //Gerador de número pseudoaleatório

    //println!("The secret number is: {secret_number}\n");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //Variável do tipo String que vai guardar o valor do input do usuário

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //Converte de String para U32(Unsigned32bits) e continua se houver input fora do padrão

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
    }
}
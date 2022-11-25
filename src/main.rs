use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1 and 10!");
    let secret_number = rand::thread_rng().gen_range(1..11);
    loop {
	print!("Please, input your number:");
	io::stdout().flush().expect("Failed to flush!");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess)
	    .expect("Failed to read line");
	let guess: u32 = guess.trim().parse()
	    .expect("ERROR: Please, type a number");
	println!("You guessed: {}", guess);
	match guess.cmp(&secret_number) {
	    Ordering::Less => println!("Too Small!"),
	    Ordering::Greater => println!("Too Big!"),
	    Ordering::Equal => {
		println!("You Win!");
		break;
	    },
	}
    }
}

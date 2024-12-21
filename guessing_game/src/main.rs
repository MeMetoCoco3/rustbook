use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    println!("Please, input your guess.");

    loop {
    let mut guess = String::new();
    // :: significa que la funcion esta ligada con algo, como un tipo.
    // io::stdin().read_line(&mut guess).expect("Failed to read line");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // Love this
    let guess_u: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>continue,
    };
   
    
    println!("You guessed: {}", guess);
    match guess_u.cmp(&secret_number){
        Ordering::Less=>println!("TooSmall!!"),
        Ordering::Equal=>println!("Nois"),
        Ordering::Greater=> {
            println!("TooBig!!");
            break;
            }
        }
    }
}

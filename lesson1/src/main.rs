use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){


    let secret_number = rand::thread_rng().gen_range(1..=100);
 
    println!("The secret_number is:{secret_number}");

    loop {
        println!("Pleaser input your guess.");

        let mut guess = String::new();

        io::stdin()
           .read_line(&mut guess)
           .expect("Failure to read inline");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        }; 
    
        println!("You guessed:{guess}");  

        match guess.cmp(&secret_number) {
             Ordering::Less => println!("Too small"),
             Ordering::Greater => println!("Too big"),
             Ordering::Equal => {
             println!("You win!");
             break;}
        }    
    }
      
}    
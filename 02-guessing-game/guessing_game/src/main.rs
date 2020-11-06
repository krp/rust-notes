use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");

    //let secret_number = rand::thread_rng();
    //let secret_number: u32 = rand::random();
    //let secret_number: f32 = rand::random();
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            //.read_line(&mut guess);
            //.read_line(guess)
            .expect("Failed to read line");
            //.expect(123);
        
        // shadow guess
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            //Err(_) => continue,  // weird syntax to see continue here.
            //Err(_) => {
            Err(x) => {
                println!("x = {}", x);
                //return ();
                //return continue;
                continue;
            },// weird syntax to see continue here.
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    //let mut guess = String::new("fds");
    //let mut guess = "fds";
    //let blorg = "hello";
    //let blah = "world";
    //println!("You guessed: {1}, {0}", blorg, blah);
    //let mut blorg = "hello";
    //let mut blah = "world";



    //blorg = "12";
    //blah = "1234";

    //let one = String::new();
    //let two = "";
    //println!("You guessed: {0}, {1}", one, two);

    //if one == two {
    //    println!("String::new() == \"\"");
    //} else {
    //    println!("String::new() != \"\"");
    //}

}

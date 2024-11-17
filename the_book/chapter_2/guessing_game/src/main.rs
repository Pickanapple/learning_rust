//Using input/output from the standard library
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut amount_of_guesses = 0;

    let secret_number = rand::thread_rng().gen_range(1..=100); //gen_range(start..=end)
    // println!("The secret number is {}", secret_number);

    //An infinite loop (like While True)
    loop {
        println!("\n\nAmount of guesses {}. Guess the number!\n\nPlease input your guess:", amount_of_guesses);
        
        //Let defines variables. The mut keyword specifies a mutable variable, which means that it can be modified
        //By default variables work out their type, but we can specify types using :i64 for example. 
        //We are letting guess be a new string
        //Guess is now equal to ""
        let mut guess = String::new();

        //If we hadn't imported std::io at the top, we could say std::io::stdin()
        io::stdin()
            //Readline method from io::stdin()
            //The & means that it is a refference. We need mut to make it mutable (chapter 4)
            //The guess is the variable we are storing it to
            .read_line(&mut guess)

            //read_line returns a result, and it has its own methods. .expect will print out the string if the input fails (result is Err)
            //The program would then also crash
            //Not including an expect method will result in a warning
            //The correct way to avoid this is error handling code (c9), but for now we just let the code crash
            .expect("Failed to read line");
        
        //We convert the guess to an unsigned 32 bit int
        //We are technically shadowing the variable (creating a new variable with the old name) - chapter 3
        let guess: u32 = match guess
            .trim() //Removes whitespace. When the user presses enter a \n character is added, which is removed here
            .parse() //Converts to a different type, which we specified in the type annotation (guess: u32)
            {
                Ok(num) => num, //If we can manage we return the number
                Err(_)  => continue, //Else we continue to the next loop
            };

        //The curly brackets show place holders, which would be substituted with guess
        //For example: 
            //let x, y, z = 1, 2, 3;
            //println!("{}, {}, {}", x, y, z);
            //Would print 1, 2, 3
    
        amount_of_guesses += 1;

        println!("\nYou guessed {}", guess);

        //The match case matches secret_number with guess based on the arms < = and >
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {println!("You win! You had {} guesses", amount_of_guesses); break} //The break keyword quits the loop
        }
    }
}
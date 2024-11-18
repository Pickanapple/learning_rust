//Variables are immutable by default, which means they can't be changed
//They can be reasigned using the let keyword again

//This would raise an error: 
//let x = 5;
//println!("{x}"");
//x = 6; //Here we are trying to change the immutable variable, which is impossible
//println!("{x}"");
//
//The mut keyword fixes this, making the variable mutable (it can be changed)

fn main() {
    let mut x = 5;
    println!("{x}");

    x = 6;
    println!("{x}");

    //We can also have constants, which can't be changed
    //They use the const keyword instead of let
    //They are named using capitals
    //Types must be annotated

    //They are calculated at compile time, not runtime
    //This means that they can only be set to a constant expression that can be found at compile time

    const THREE_HOURS_IN_SECONDS: i32 = 3 * 60 * 60;
    println!("{}", THREE_HOURS_IN_SECONDS);

    //THREE_HOURS_IN_SECONDS = 2; would not compile (changing a constant)
    // const THREE_HOURS_IN_SECONDS: i32 = 5; Would not compile either

    //Shadowing

    //If we make a new variable with the same name as the old variable, it is called shadowing
    let shadow = "This is a variable";
    println!("{shadow}");

    let shadow = "This is a shadowed variable";
    println!("{shadow}");

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of the inner scope is {x}"); //x is 12 inside the scope
    }

    println!("The value of x is {x}"); //x is now 6

    //As we are creating a new variable, we can change the type
    let spaces = "          ";
    println!("{spaces}");

    let spaces = spaces.len();
    println!("{spaces}");

    //This would not work: 
    //let mut spaces = "     ";
    //let spaces = spaces.len();

}
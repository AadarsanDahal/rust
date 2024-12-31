//steps to solve the problem
// printing and formatting

use std::array;

fn main() {
    println!("hello this is day 2 of rust ");

    println!("my name is ? and i love robots ");
    let name = "Aarsan";
    //basic formatting
    println!("my name is {} and i love robots ", name);
    println!("my name is {} and i love {} ", "Jhon", "glasses");
    println!("my name is {} and i love robots ", name);
    println!("{}", 4);

    // for eg place holders

    println!(
        "my best frinend is {} and she is {} years old",
        "aaisha", 17
    );
    // i dont want to write name twice so i will try to use use 0 and 1 as counting
    println!(
        "my best frinend is {0} and {0} is {1} years old",
        "aaisha", 17
    );

    // named arguments adding

    println!(
        "my name is {name} and i am {age}",
        name = "Bandri",
        age = 18
    );

    //tuples

    let student = ("aadarsan", 19, true);
    println!(" my name is {:?} ", student);

    //basic mathmatics |o|
    println!("binary {:b}", 5); // gives a binaryy tings

    //adding numbers
    //implicit (rust automatically fgures it out)
    let x = 12;
    let y = 3;

    let z = x + y;
    // just learned that sum cant be used llowl caues thats pre defeiced
    println!("the sum is {z}");
    println!(" is {}", 12 + 3);
    println!("the product are is {}", 12 * 3);
    println!("the product are is {}", 12 / 3);
    println!("the product are is {}", 12 + 3);
    println!("the product are is {}", 12 - 3);

    // rust formatting
    //rustfmt main.rs

    //variables //
    //calling functions
    variables();
}
//created a function and called it inside the main one
fn variables() {
    /*this is how to print a variable here we
    are using eplicit (we need to specify the variable types) */

    let a: u32 = 5;
    println!("{}", a);

    // by default in rust  all variable are immutable
    //MUTABILITY
    let mut j = 10;

    println!(" not changed {}", j);
    j = 100;
    println!("cahnged {}", j);

    /*Let twice method writting let before the variable , which accepts different data tpyes where mut fails to do so  */

    let (name, tall, hobbie) = ("aadarsan", true, "codingg");
    println!(
        "my name is {} and i am {} also i love {} ",
        name, tall, hobbie
    );

    //Constants
    /*mut dosent aply here , once its there its there */
    const PI: f32 = 3.14;
    println!("{PI}");

    //Shadowing
    let mut b = 3;
    println!("{}", b);

    //here we defined a scope for b
    {
        b = 4;
        println!("{b}");
    }

    // Day 3 ended

    // This tells you the maximum and minimum of an i32

    println!("max i32 is {}", std::i8::MIN);
    println!("max i32 is {}", std::i8::MAX);

    //Floating points types
    //they can only be f34 or f64
    let y: f64 = 10.14; //default being f64
    println!("{y}");

    /*BOOLEANS  True of False*/
    let true_false = true;
    println!("{true_false}"); //gives result true

    /*CHARACTERS */
    //only consist of 1 letter
    let char: char = 'a';
    println!("{char}");

    let emoji: char = '\u{1F618}';
    println!("{emoji}");

    /* COMPOUND TYPES */

    /*TUPLES */
    let mut tup1 = (18, "Dahal", true);
    println!("{:?}", tup1);
    //changing the value of a mutable tuple , but need to rember that needs to be same data tpyes
    tup1.0 = 2095;
    tup1.1 = "bandri";
    tup1.2 = false;
    //printing the tuple
    println!("{:?}", tup1);

    let mut arr = [10, 20, 30];
    // Unlike any other programming language, you can't add any extra elements
    // onto this array :( BUT there's something called 'vectors' which
    // allow you to do this and we'll touch upon them later!
    // Modifying the elements
    println!("{:?}", arr);
    //changing the values of an array
    arr[0] = 1000;
    arr[1] = 1000;
    arr[2] = 1000;
    //printing the values of an array which is changed

    println!("{:?}", arr);

    // end of day 4
}

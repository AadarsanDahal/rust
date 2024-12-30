//steps to solve the problem
// printing and formatting

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
    let mut  j=10;

    println!(" not changed {}",j);
    j=100;
    println!("cahnged {}",j);

    /*Let twice method writting let before the variable , which accepts different data tpyes where mut fails to do so  */
    

    let (name ,tall,hobbie)=("aadarsan",true,"codingg");
    println!("my name is {} and i am {} also i love {} ",name,tall,hobbie);


    //Constants
    /*mut dosent aply here , once its there its there */
    const PI :f32 = 3.14;
    println!("{PI}");


    //Shadowing
    let mut b = 3;
    println!("{}",b);
    
    //here we defined a scope for b 
    {
        b=4;
        println!("{b}");
    }

}

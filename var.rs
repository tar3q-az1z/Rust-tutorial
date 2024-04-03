fn main(){
    // immutable variables
    let x = 1; // int 
    let y = 23.34; // float
    let msg = "text me"; // string
    println!("x = {x}, y = {y}, msg = {msg}");

    // mutable variable declaration
    let mut age = 5;
    println!("age = {age}");
    age = 39; 
    println!("age = {age}");

    // rust constants 
    const PI:f32 = 3.14;
    println!("PI = {PI}");
}
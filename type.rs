fn main(){

    // integer
    let x: i32 = -5;
    println!("x = {x}");
    let y: u32 = 8;
    println!("y = {y}");
    let z: i128 = 34743477487387434834445445482144554889; // upto 38 digits
    println!("z = {z}");
    let mut n: i64 = 3343434343430; // upto 18 digits
    println!("n = {n}");
    n = 9999999;
    println!("n = {n}");

    // float
    let r: f32 = 3.343;
    let pi: f64 = 3.1434;
    println!("pi = {pi}");
    println!("r = {r}");
    
    // boolean
    let f1: bool = true;
    let f2: bool = false;
    println!("f1 = {f1}");
    println!("f2 = {f2}");

    // character
    let ch: char = 'Z';
    println!("ch = {ch}");

    let p = 332; // default type is i32
    println!("p = {p}");
}
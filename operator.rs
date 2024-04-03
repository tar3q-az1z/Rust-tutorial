fn main(){
    // arithmetic operations
    let mut x = 32;
    let y = 5;
    println!("{x} + {y} = {}", x + y);
    println!("{x} - {y} = {}", x - y);
    println!("{x} x {y} = {}", x * y);
    println!("{x} / {y} = {}", x / y);
    println!("{x} % {y} = {}", x % y);
    let dv: f64 = x as f64 / y as f64; // real division
    println!("dv = {dv}");
    
    // compound 
    x += 10;
    println!("x = {x}");

    // comparison

    println!("x > y: {}", x > y);
    
    // logical 
    println!("x > y && x > 100: {}", x > y && x > 100);
    println!("!x>0: {}", !x > 0);
}
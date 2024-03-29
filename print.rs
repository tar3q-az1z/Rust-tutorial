fn main(){
    // let print string
    println!("That's it!");
    print!("Learn rust. ");
    print!("Have Fun!!!");

    // let print variable
    let x = 1;
    println!("");
    println!("x = {}", x);
    print!("x + 1 = {}", x + 1);
    
    // printing multiple variables
    
    let y = 2;
    print!("\n");
    println!("x = {}, y = {}", x, y);
    println!("y = {1}, x = {0}", x, y);
    println!("x = {x}, y = {y}"); // that's easy, gonna with it
    // all four are same
}
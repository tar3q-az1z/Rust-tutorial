fn main(){
    let tax: f32 = 34534.343;
    let pay = tax as i32; // float to int casting
    println!("tax: {}", tax);
    println!("pay: {}", pay);

    let ch: char = 'A';
    let chtoint: i8 = ch as i8;
    println!("ch: {ch}");
    println!("chtoint: {}", chtoint);

    let f: bool = true; 
    let m: i32 = f as i32; // true turns to 1
    println!("f: {}", f);
    println!("m: {}", m);
}
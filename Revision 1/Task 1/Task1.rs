use std::io::{self,Write};
fn main(){
    println!("Enter First Number: ");
    io::stdout().flush().unwrap();
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let f_n:f64=input.trim().parse().expect("Enter a valid float");


    println!("Enter Second Number: ");
    io::stdout().flush().unwrap();
    input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let i_n:i32=input.trim().parse().expect("Enter a valid integer");

    println!("Addition: {:.3}",(f_n+i_n as f64));
    println!("Subtraction: {:.3}",(f_n-i_n as f64));
    println!("Multiplication: {:.3}",(f_n*i_n as f64));
    println!("Division: {:.3}",(f_n/i_n as f64));

}
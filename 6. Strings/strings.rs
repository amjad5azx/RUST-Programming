fn main(){
    println!("\nString Literals");
    println!("---------------");

    let mut message=String::from("Earth");
    println!("Message: {}",message);

    message.push_str(" is home");
    println!("Message: {}",message);
}
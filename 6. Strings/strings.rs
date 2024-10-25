fn main(){
    println!("\nString Literals");
    println!("---------------");

    let mut message=String::from("Earth");
    println!("Message: {}",message);

    message.push_str(" is home");
    println!("Message: {}",message);

    println!("\nSlice:");
    println!("------");
    let name="Muhammad Amjad";
    println!("Full name is {}",&name);
    println!("Last name is {}",&name[9..9+5]);

    let num=[1,2,3,4,5,6,7,8];
    //let inner_num:&[i32]=&num[..4];
    println!("Numbers are {:?}",&num[..4]);

    let message=String::from("Hello World!");
    let first_word=get_first_word(&message);
    println!("First Word is {}",first_word);

    let text = "Hello";
    let bytes = text.as_bytes(); // [72, 101, 108, 108, 111]
    println!("Bytes values are: {:?}",bytes);

    let space = b' '; // Equals 32
    println!("Space value is {:?}",space);

    println!("Trim Spaces Challenge");
    println!("---------------------");
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from(" There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");
    
    let test4 = "We're surrounded by space!";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = "";
    assert_eq!(trim_spaces(test7), "");

    println!("Tests passed!");

}
fn get_first_word(s:&String)->&str{
    let bytes=s.as_bytes();

    for (index,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..index];
        }
    }
    &s
}

fn trim_spaces(inp:&str)->&str{
    inp.trim()
}
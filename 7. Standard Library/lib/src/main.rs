use std::io;
use rand::prelude::*;

fn main(){
   /* let mut buffer=String::new();
    println!("Enter message");
    let _=io::stdin().read_line(&mut buffer);
    println!("Mut buffer is {}",buffer);

    let num:i32=buffer.trim().parse().unwrap();
    /*
    trim(): This function removes any leading and trailing whitespace (like spaces, newlines, etc.) from the buffer string.
    parse(): This function converts the trimmed string into a specified data type, in this case, an integer (i32).
    unwrap(): This function is used to get the actual value from the result of parse(). If parsing fails, unwrap() will cause the program to panic (stop running with an error).
     */
    println!("Number is {}",num+1);
    */

    println!("Random Number");
    let ran_num=random::<f64>();
    println!("Number is {}",ran_num);
    let ran_num=thread_rng().gen_range(1..11);
    println!("Number is {}",ran_num);

    println!("High or Low Challenge");
    println!("---------------------");

    let ran_num=thread_rng().gen_range(1..100);


    loop
    {
        let mut guess=String::new();
        println!("Enter number: ");
        let _=io::stdin().read_line(&mut guess);
        let g_num:i32=guess.trim().parse().unwrap();
        if g_num>100 || g_num<1{
            println!("Number out of range. Try again");
            continue;
        }
        if g_num>ran_num{
            println!("Number is greater");
        }
        if g_num<ran_num{
            println!("Number is lesser");
        }
        if g_num==ran_num{
            println!("Number is guessed correctly");
            break;
        }
    }
}
fn main(){
    let letters=['q','v','g','f','l','a','c'];
    let mut count=0;
    loop{
        println!("Letter: {}",letters[count]);
        if letters[count]=='l'{
            break;
        }
        count+=1;
    }
    let mut v=0;
    let mut c=0;
    count=0;
    while count<letters.len(){
        if letters[count]=='a' || letters[count]=='e' || letters[count]=='i' || letters[count]=='o' || letters[count]=='u'{
            v+=1;
        }
        else{
            c+=1;
        }
        count+=1;
    }
    if c>v{
        println!("It has less vowels");
    }
    else{
        println!("It has less consonants");
    }

    println!("\nFactorial Using While Loop:");
    println!("---------------------------");
    let mut num=5;
    let mut fact=1;
    while num>0{
        fact*=num;
        num-=1;
    }

    println!("Factorial: {}",fact);

    println!("\nFactorial Using For Loop:");
    println!("---------------------------");
    num=5;
    fact=1;
    for i in 1..=num{
        fact*=i;
    }
    println!("Factorial: {}",fact);

}
fn main(){
    println!("\nConditional Expression");
    println!("----------------------");
    let c_val=0;
    if c_val>0{
        println!("Number is positive");
    }
    else if c_val<0{
        println!("Number is negative");
    }
    else{
        println!("Number is Zero");
    }

    let c_val_make=true;
    let x_val=if c_val_make {1} else {2}; //if-else expression 
    println!("x val is: {}",x_val);

    println!("\nLoop: ");
    println!("--------");
    let mut count=0;
    let result=loop {
        count+=1;
        println!("Count: {}",count);
        if count==10{
            break count*10;
        }
    };
    println!("Loop Result: {}",result);

    println!("\nWhile Loop: ");
    println!("--------");
    count=0;
    while count<10{
        count+=1;
        println!("Count: {}",count);
    }
    
    let  _n_letters=['a','b','c'];
    count=0;
    while count<_n_letters.len(){
        println!("Letter is {}",_n_letters[count]);
        count+=1;
    }
    //count=0;
    println!("For Loop");
    println!("--------");
     let message=['h','e','l','l','o'];
     for item in message{
        println!("item is {}",item);
     }

     for (index,item) in message.iter().enumerate(){
        println!("item {} is {}",index,item);
     }

     for (index,&item) in message.iter().enumerate(){
        println!("item {} is {}",index,item);
        if item=='l'{
            break;
        }
     }

     for numbers in 0..5{
        println!("Number is {}",numbers);
     }

     println!("\nMinimum, Maximum, Mean");
     println!("----------------------");
     let num_arr = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
     let mut min=num_arr[0];
     let mut max=num_arr[0];
     let mean:f64;
     let mut sum=0;
     for &item in num_arr.iter(){
        if min>item{
            min=item;
        }
        if max<item{
            max=item;
        }
        sum+=item;
     }
     mean=sum as f64/num_arr.len() as f64;
     println!("Min: {}",min);
     println!("Max: {}",max);
     println!("Mean: {}",mean);
}
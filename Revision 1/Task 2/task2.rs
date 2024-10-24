fn main(){
    let mut arr=[7,3,2,7,4,12,65,15,78,90];

    arr[5]=111;
    arr[2]=98;
    arr[3]=144;

    println!("Array: ");
    println!("-------");
    for (ind,item) in arr.iter().enumerate(){
        println!("Element at index:{} is {}",ind+1,item);
    }

    let tup=(12,33.5,'a',55.76);

    println!("\nTuple Indexing");
    println!("--------------");
    println!("Element at Index {} is {}",0,tup.0);
    println!("Element at Index {} is {}",1,tup.1);
    println!("Element at Index {} is {}",2,tup.2);
    println!("Element at Index {} is {}",3,tup.3);

    println!("\nTuple Destructing: ");
    println!("-------------------");
    let (a,b,c,d)=(12,33.5,'a',55.76);
    println!("Element a: {}",a);
    println!("Element b: {}",b);
    println!("Element c: {}",c);
    println!("Element d: {}",d);

    println!("\nSum, Min, Max, Mean:");
    println!("----");
    let mut sum=0;
    let mut min=arr[0];
    let mut max=arr[0];
    for item in arr{
        sum+=item;
        if min>item{
            min=item;
        }
        if max<item{
            max=item;
        }
    }
    println!("Sum:{}",sum);
    println!("Mean: {}",sum as f64/arr.len() as f64);
    println!("Minimum: {}",min);
    println!("Maximum: {}",max);

}
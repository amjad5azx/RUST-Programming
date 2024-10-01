fn main(){
    let _tup=(10,3.14,'z');
    let mut _tup2: (i32,f64,char)=(10,3.14,'z');
    let (_t1,_t2,_t3)=_tup2;


    sum_of_two_numbers(_tup.1,_tup2.1);
    println!("Square is: {}",square(3 as f64));
    println!("Square with multiple value is: {:?}",square_with_multiple(3 as f64,4 as f64));

    println!("\nCelcius to Farentheit");
    println!("---------------------");
    let celcius_temp=23.0;
    let farenheit_temp=celcius_to_farenheit(celcius_temp);
    println!("Celcius to farenheit tempetature is: {}",farenheit_temp);
}

// functions
fn sum_of_two_numbers(x:f64,y:f64){
    println!("Sum of two numbers: {}",(x+y))
}
fn square(x:f64)-> f64{
    x*x
}

fn square_with_multiple(x:f64,y:f64)-> (f64,f64){
    return (x*x,y*y);
}

fn celcius_to_farenheit(temp_c:f64)->f64{
    return 1.8*temp_c+32.0;
}
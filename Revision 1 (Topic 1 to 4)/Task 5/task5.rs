fn main(){
    let res=calculate(3,4.4);
    println!("Result({:.3},{:.3},{:.3})",res.0,res.1,res.2);
    println!("Factorial: {}",factorial(5));

    println!("Result: {}",apply_operation(34,11,"/"));
}
fn calculate(num1:i32,num2:f64)->(f64,f64,f64){
    return (num1 as f64+num2,num1 as f64-num2,num1 as f64*num2);
}

fn factorial(num:i32)->i32{
    if num==0{
        return 1;
    }
    else{
        return num*factorial(num-1);
    }
}

fn apply_operation(n1:i32,n2:i32,op:&str)->i32{
    let mut oper=0;
    if op=="+"{
        oper=n1+n2; 
    }
    if op=="-"{
        oper=n1-n2; 
    }
    if op=="*"{
        oper=n1*n2; 
    }
    if op=="/"{
        oper=n1/n2; 
    }
    return oper
}
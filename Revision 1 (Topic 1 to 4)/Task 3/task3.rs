fn main(){
    let value1:i32=0b1010_1100;
    let value2:i32=0b1101_0010;
    println!("{}\n{}",value1,value2);

    println!("\nDecimal Format Result:");
    println!("----------------------");
    println!("Value1 AND Value2: {}",value1&value2);
    println!("Value1 OR Value2: {}",value1|value2);
    println!("Value1 XOR Value2: {}",value1^value2);
    println!("NOT Value 1: {}",!value1);
    println!("LEFT SHFT Value 1: {}",value1<<2);
    println!("RIGHT SHIFT Value 1: {}",value1>>2);
    println!("NOT Value 2: {}",!value2);
    println!("LEFT SHFT Value 2: {}",value2<<2);
    println!("RIGHT SHIFT Value 2: {}",value2>>2);

    println!("\nBinary Format Result:");
    println!("----------------------");
    println!("Value1 AND Value2: {:08b}",value1&value2);
    println!("Value1 OR Value2: {:08b}",value1|value2);
    println!("Value1 XOR Value2: {:08b}",value1^value2);
    println!("NOT Value 1: {:08b}",!value1);
    println!("LEFT SHFT Value 1: {:08b}",value1<<2);
    println!("RIGHT SHIFT Value 1: {:08b}",value1>>2);
    println!("NOT Value 2: {:08b}",!value2);
    println!("LEFT SHFT Value 2: {:08b}",value2<<2);
    println!("RIGHT SHIFT Value 2: {:08b}",value2>>2);

    println!("Boolean Values");
    println!("--------------");
    let b1=true;
    let b2=false;

    println!("AND: {}",b1&b2);
    println!("OR: {}",b1|b2);
    println!("XOR: {}",b1^b2);
    println!("NOT: {}",!b1);
    println!("NOT: {}",!b2);




}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}


fn main(){
    println!("Integer Data Type: ");
    println!("-------------------------------");

    let mut x=10;
    println!("x: {}", x);
    //if we do not change the value here and have used mut keyword, this will give an error. For checking this you can comment the code below
    x=20;
    println!("x: {}", x);
    x+=1;
    println!("x: {}", x);

    println!("\nFloating Data Type: ");
    println!("-------------------------------");

    let y=10.1232132131233213;
    println!("y: {}",y);

    let y2 :f32=10.1232132131233213;
    println!("y2: {}",y2);

    println!("\n\nArithemation Operations: \nIntegers: \n");
    println!("-------------------------------");


    let mut a=10;
    let mut b=3;
    println!("Addition: {}",(a+b));
    println!("Subtraction: {}",(a-b));
    println!("Multiplication: {}",(a*b));
    println!("Division: {}",(a/b));
    println!("Modulus: {}",(a%b));

    println!("\nFloating-Point: \n");
    println!("-------------------------------");

    let a1=10.0;
    let b1=3.0;

    println!("Addition: {}",(a1+b1));
    println!("Subtraction: {}",(a1-b1));
    println!("Multiplication: {}",(a1*b1));
    println!("Division: {}",(a1/b1));
    println!("Modulus: {}",(a1%b1));

    println!("\nConverting Integer to float:");
    println!("-------------------------------");

    let res=a as f64 /b1;// "as" keyword used to convert from one type to other type
    println!("Result in integer: {}",res as i64);
    println!("Result in float: {}",res as f64);

    println!("\nFormatting Print Statement: ");
    println!("-------------------------------");
    println!("y: {:.3}",y); //The .3 means that the number should be displayed with 3 digits after the decimal point.
    println!("y: {:8.3}",y);//The number 8 specifies that the output should occupy at least 8 characters.
    println!("y: {:08.3}",y);//If the number is shorter, it will be padded with leading zeros (0) to make it 8 characters long.
    println!("x:{0}\ty:{1}",x,y);//explicit index
    println!("x:{}\ty:{}",x,y);//implicit index
    println!("Person: {person}",person="amjad");//named


    println!("\nBitwise Operator: ");
    println!("-------------------------------");

    let value = 0b1111_0101u8;  
    //Underscore is used to clarity
    //0b is used to indicate that the number is written in binary (base-2) format.
    //assigns it the binary number '1111_0101' (equivalent to 245 in decimal).

    println!("value is {}", value);  

    println!("value is {:08b}", value);  
    // Prints the binary representation of 'value', formatted to 8 bits, including leading zeros.
    // {:08b} means:
    // - `b`: Display in binary.
    // - `08`: Pad with leading zeros to ensure the output is 8 characters long.

    println!("Bitwise Not: {:08b}",!value);
    println!("Bitwise And: {:08b}",(value & 0b1001_0101));
    println!("Bitwise Or:  {:08b}",(value | 0b1001_0101));
    println!("Bitwise Xor: {:08b}",(value ^ 0b1001_0101));
    println!("Bitwise Left Shift by 2 bits: {:08b}",(0b10010101 << 2));
    println!("Bitwise Right Shift by 2 bits: {:08b}",(0b10010101 >> 2));

    println!("Boolean Data Types and its Operations:");
    println!("--------------------------------------");
    let bool_a=true;
    let bool_b=false;
    println!("a: {}, b:{}",bool_a,bool_b);
    println!("Not a is: {}", !bool_a);
    println!("Not b is: {}",!bool_b);
    println!("a AND b is: {}",bool_a & bool_b);
    println!("a OR b is: {}",bool_a | bool_b);
    println!("a XOR b is: {}",bool_a ^ bool_b);
    let bool_res=(bool_a^bool_b) | (bool_a&bool_b);
    println!("Result is :{}",bool_res);

    println!("Comparison Operators: ");
    println!("----------------------");

    a = 1;
    b = 2;
    let _test=2.0;
    println!("a is {} and b is {}", a, b);
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);
    //if either a or b are of different data types, you will get error

    println!("Character Data Type: ");
    println!("---------------------");
    let letter='a';
    let number='b';
    let white_chess_pawn='\u{2659}';
    println!("{}\n{}\n{}",letter,number,white_chess_pawn);

    println!("Challenge: Calculating Average");
    println!("------------------------------");

    let ch_a=13;
    let  ch_b=2.3;
    let ch_c:f32=120.0;
    let average= (ch_a as f64 + ch_b  + ch_c as f64)/3 as f64;
    assert_eq!(average,45.1);
    println!("Test Passed");

    println!("Arrays: ");
    println!("--------");
    let mut letters=['a','b','c'];
    letters[0]='x';
    let first_letter=letters[0];
    println!("First Letter: {}",first_letter);

    let numbers : [i32; 5];//Declaring array. i32 is data type and 5 is length
    numbers=[0;5];
    println!("last number is {}",numbers[4]);
    println!("Length of Array: {}",numbers.len());
    let leng=numbers.len();
    print_type_of(&leng);

    println!("Multi Dimensional Arrays: ");
    println!("-----------");
    let double_array=[
        [1,2,3],
        [4,5,6]
    ];
    println!("Data: {}",double_array[0][2]);
    println!("Data: {}",double_array[1][1]);

    let mut _array:[[[i32;100];20];5];// Declaring 3d array
    _array=[[[0;100];20];5];//3d array with zeroes
    println!("3d Array: {}",_array[0][2][3]);

    println!("Tuples: ");
    println!("--------");
    let _tup=(10,3.14,'z');
    println!("Third Item: {}",_tup.2);
    let mut _tup2: (i32,f64,char)=(10,3.14,'z');
    _tup2.0+=12;
    _tup2.1+=12.0;
    println!("Tuple: ({},{},{})",_tup2.0,_tup2.1,_tup2.2);
    let (_t1,_t2,_t3)=_tup2;
    println!("t1: {}, t2:{}, t3:{}",_t1,_t2,_t3);

}
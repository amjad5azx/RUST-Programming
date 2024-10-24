fn main(){
    println!("\nShadowing");
    println!("---------");
    let planet="Earth";
    println!("Planet is Earth: {}",planet);

    let planet="Venus";
    println!("Planet is Earth: {}",planet);

    let planet=5;
    println!("Planet is Earth: {}",planet);

    println!("\nOwnership");
    println!("---------");

    let outer_planet: String;
    {
        let inner_planet = String::from("Mercury");
        outer_planet = inner_planet; // Moves `inner_planet` to `outer_planet`
        //println!("Inner planet is: {}", inner_planet); // This will cause an error since inner palnet no longer owns the data
    }
    println!("Outer planet is: {}", outer_planet);

    println!("Transfering Ownership Example 2");
    println!("-------------------------------");
    let s1 = String::from("Hello"); // s1 owns the String
    let s2 = s1; // Ownership is moved to s2, s1 is no longer valid

    //println!("{}", s1); // This will cause an error since s1 no longer owns the data
    println!("{}", s2); // s2 can use the String

    println!("\nBorrowing Refernces");
    println!("-------------------");

    println!("\nExample 1:");
    let s1 = String::from("Hello");
    let len = calculate_length(&s1); // Pass a reference to s1
    println!("Length of '{}' is {}", s1, len); // s1 can still be used 

    println!("\nExample 2:");
    let num = 42; // Integer variable
    let doubled_value = double_value(&num); // Borrow a reference to num
    println!("Original number: {}", num); // num can still be used
    println!("Doubled value: {}", doubled_value);

    println!("\nMutable Refernces");
    println!("-------------------");
    println!("Example 1: ");
    let mut s = String::from("Hello");
    change(&mut s); // Mutable reference

    println!("{}", s);

    println!("Example 2: ");
    let mut x = 5;
    change_value(&mut x);
    println!("x is: {}", x);

    println!("Important Point:");

    let mut x = 10;

    // Rule 1: Only one mutable reference is allowed at a time.
    let x_mut_ref1 = &mut x; // ✅ This is allowed.

    // let x_mut_ref2 = &mut x; // ❌ Uncommenting this line would cause a compile error because we can't have two mutable references at the same time.

    *x_mut_ref1 += 5;
    println!("Mutable reference 1: {}", x_mut_ref1);

    // After `x_mut_ref1` is no longer used, we can create another mutable reference.
    let x_mut_ref2 = &mut x;
    *x_mut_ref2 += 5;
    println!("Mutable reference 2: {}", x_mut_ref2);

    // Rule 2: Multiple immutable references are allowed at the same time.
    let x_imm_ref1 = &x; // ✅ Immutable reference 1
    let x_imm_ref2 = &x; // ✅ Immutable reference 2
    println!("Immutable references: {} and {}", x_imm_ref1, x_imm_ref2);

    // Rule 3: You cannot mix mutable and immutable references.
    // let x_mut_ref = &mut x; // ❌ Uncommenting this will cause an error because we can't have a mutable reference while immutable references are active.

    println!("\nDangling Refernces");
    println!("-------------------");
    //let _reference = dangle(); // This will not compile in Rust

}

fn double_value(n: &i32) -> i32 {
    *n  // Dereference n
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn change_value(num: &mut i32) {
    *num += 1;
}

/*fn dangle() -> &String { 
    let s = String::from("Hello");
    &s // Error! Returning a reference to `s`, which is dropped after the function
}*/
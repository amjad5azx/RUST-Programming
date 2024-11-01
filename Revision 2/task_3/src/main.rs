fn main() {
    let greeting = String::from("Konnichiwa");

    // Call both functions with an immutable reference to `greeting`
    let modified1 = borrow_and_return1(&greeting);
    let modified2 = borrow_and_return2(&greeting);

    // Print the returned modified strings
    println!("Modified greeting 1: {}", modified1);
    println!("Modified greeting 2: {}", modified2);

    // Print the original `greeting` to confirm it remains unchanged
    println!("Original greeting: {}", greeting);
}

fn borrow_and_return1(x: &String) -> String {
    // Create and return a modified version of `x`
    format!("{} - Hello from function 1!", x)
}

fn borrow_and_return2(x: &String) -> String {
    // Create and return a modified version of `x`
    format!("{} - Greetings from function 2!", x)
}

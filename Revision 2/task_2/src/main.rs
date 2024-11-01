fn main() {
    let name=String::from("Amjad");
    println!("{}",name);
    let new_name=process_task(name);
    println!("{}",new_name);
    // println!("Name: {}",name)
}

fn process_task(mut x:String)->String{
    x.insert_str(0,"Muhammad ");
    x
}

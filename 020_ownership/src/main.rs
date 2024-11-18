fn main() {
    let value = vec![1, 2, 3];
    println!("value is {value:?}");
    
    let other_value = value;
    println!("other_value is {other_value:?}");
    
    //println!("value is {value:?}");
    
    let mut value = generate_vector();
    print_vector(&value); // Read-only
    println!("value is {value:?}");

    manipulate_vector(&mut value);
    println!("value is {value:?}");
}

fn generate_vector() -> Vec<i32> {
    let value = vec![1, 2, 3];
    value
}

fn print_vector(value: &Vec<i32>) {
    println!("value is {value:?}");
}

fn manipulate_vector(value: &mut Vec<i32>) {
    value.push(4);
}
fn multiply_by_five(number: Option<i32>) -> Option<i32> {
    number.map(|x| x * 5)
}

fn main() {
    println!("multiplied by 5: {}", multiply_by_five(Some(5)).unwrap());
    println!("multiplied by none: {}", multiply_by_five(None).unwrap_or_default());
}

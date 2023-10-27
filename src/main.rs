fn practice(numbers: Vec<usize>, index: usize) -> usize {
    numbers.get(index).unwrap_or(&index) * 5
}

fn main() {
    println!("index 0: {}", practice(vec![1, 2, 3], 0));
    println!("index 10: {}", practice(vec![1, 2, 3], 10));
}

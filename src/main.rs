fn main() {
    let list = vec![1, 2, 3];

    let result: Vec<_> = list.iter().map(|x| x + 1).collect();

    println!("{:?}", result);
}

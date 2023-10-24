fn main() {
    let list = vec![1, 2, 3];

    let mut foo = list.iter().map(|x| x + 1);

    let mut new_vector = vec![];

    while let Some(x) = foo.next() {
        new_vector.push(x);
    }

    println!("{:?}", new_vector);
}

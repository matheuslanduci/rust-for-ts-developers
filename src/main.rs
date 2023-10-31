#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(item: &mut Item) {
    item.count += 1;
}

fn print_all(items: &Vec<Item>) {
    for item in items {        
        println!("{:?}", item);
    }
}

fn main() {
    let mut items = vec![Item { count: 1 }];

    let first = items.get_mut(0);
    let second = items.get_mut(1);

    println!("{:?}", second);

    print_all(&items);
}
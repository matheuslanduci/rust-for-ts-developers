fn main() {
    let file = std::env::args()
        .nth(1)
        .expect("The file name should be present");

    let content = std::fs::read_to_string(file).expect("Ooops, the file does not exist!");

    content.lines().for_each(|line| {
        if let Ok(value) = line.parse::<usize>() {
            println!("{}", value)
        } else {
            println!("Line not a number");
        }
    })
}

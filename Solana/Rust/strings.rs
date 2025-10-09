fn main() {
    let greeting: String = String::from("hello world");
    pringln!("{}", greeting);

    let char1 = greeting.chars().nth(1000);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("not found"),
    }

    print!("{}", char1.unwrap()); // unwrap is not usuallty safe to use, whenever we use unwrap we say the comipler that we are olkay with the risk of getting None.
}

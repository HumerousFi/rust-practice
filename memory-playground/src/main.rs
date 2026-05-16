fn main() {
    let s = String::from("Hello World");

    let slice = &s[0..5];

    println!("Original: {}", s);
    println!("Slice: {}", slice);
}

fn main() {
    let mut s = String::from("hello", 0);

    s.push_str(", world!"); // some comment

    println!("{}", s);
}

#![warn(unused_variables)]
fn main() {
    let s1 = "hello world 🦀"; // stored in binary

    let mut s2 = String::from("hello world");
    s2.push_str(" bar"); // push on to string
    println!("{}", s2);
    s2.replace_range(.., "hello"); // replace on to string

    let s3 = "hello world".to_string();
    let s4 = "hello world".to_owned();

    // concatenations of Strings
    let s6 = s3 + &s4;
    println!("{}", s6);

    let s7 = ["first", "second"].concat();
    println!("{}", s7);

    let s8 = concat!("1st", "2nd");
    println!("{}", s8);

    let s5 = &s4[..];

    println!("{}", s5);
}

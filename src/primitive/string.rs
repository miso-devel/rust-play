#[allow(dead_code)]
fn main() {
    println!("primitive/string!!!");
    let s1: String = String::from("Hello World!!");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
}

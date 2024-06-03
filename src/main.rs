fn main() {
    let mut s = String::from("This is a ");
    string_adder(&mut s);
    println!("{}", s);

    let reference = &s;
    println!("inmutable reference: {}", reference); 

}

fn string_adder (s: &mut String) {
    s.push_str("string");
}

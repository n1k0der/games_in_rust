#[derive(Debug)]
enum SoldierType {
    Arquero,
    Infante,
    Caballero,    
}

#[derive(Debug)]
struct Soldier {
    soldier_type: SoldierType,
}

impl Soldier {
    fn new (st: SoldierType) -> Soldier{
        Soldier{soldier_type:st}
    }

}

fn main() {
    // let mut s = String::from("This is a ");
    // string_adder(&mut s);
    // println!("{}", s);

    // let reference = &s;
    // println!("inmutable reference: {}", reference); 

    let primer_recluta = Soldier::new(SoldierType::Arquero);
    println!("{:?}, {:?}", primer_recluta, primer_recluta.soldier_type);
}

// fn string_adder (s: &mut String) {
//     s.push_str("string");
// }

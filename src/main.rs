use simulator::society::Society;
fn main() {
    let society = Society::new(10000);
    print!("{:?}", society.citizens[30].id)

}
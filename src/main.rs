use qi_core;

fn main() {
    let result = qi_core::parse_from_file("./playground/other.qi", qi_core::parsers::atom::parse);

    println!("{:#?}", result)
}

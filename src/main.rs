use qi_core;

fn main() {
    let result = qi_core::parse_from_file(
        "C:\\Users\\Zana\\Documents\\code\\qi\\core\\demo_ignore\\playground\\other.qi",
        qi_core::parsers::atom::parse,
    );

    println!("{:?}", result)
}

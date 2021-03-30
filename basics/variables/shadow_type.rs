fn main() {
    let spaces = "     "; // inferred type String
    println!("Spaces: [{}]", spaces);
    // spaces on rhs is the String spaces. spaces on lhs is u32 and shadows the earlier one.
    let spaces: usize = spaces.len();
    println!("# of spaces: {}", spaces);
}

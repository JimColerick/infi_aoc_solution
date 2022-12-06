use infi::do_part_1;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Manhattan distance -> {}", do_part_1(&input));
}

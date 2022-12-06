use infi::do_part_2;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let painted_points = do_part_2(&input);
    for y in (0..10).rev() {
        for x in 0..70 {
            if painted_points.contains(&(x,y)) {
                print!("*");
            }
            else {
                print!(" ");
            }
        }
        println!();
    }
}

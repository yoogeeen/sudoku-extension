mod puzzle;

fn main() {
    let puzzle = puzzle::create_puzzle();
    puzzle::print_puzzle(puzzle);
}

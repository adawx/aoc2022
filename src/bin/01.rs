fn main() {
    let input = include_str!("../inputs/01.txt");

    let lines = input.split("\n\n");

    let mut parsed_lines: Vec<u32> = lines
        .map(|line| {
            line.split("\n")
                .flat_map(|num| num.parse::<u32>())
                .sum::<u32>()
        })
        .collect();

    parsed_lines.sort_by(|a, b| b.cmp(a));

    // Part 1
    println!("{:?}", parsed_lines[0]);
    // Part 2
    println!("{:?}", parsed_lines.iter().take(3).sum::<u32>());
}

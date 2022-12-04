fn main() {
    let input = std::fs::read_to_string("./src/inputs/04.txt").unwrap();

    let lines: Vec<&str> = input.lines().collect();

    // Part 1
    let mut count = 0;
    for line in &lines {
        let ranges: Vec<&str> = line.split(',').collect();
        let range1: Vec<i32> = ranges[0]
            .split('-')
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let range2: Vec<i32> = ranges[1]
            .split('-')
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        if (range1[0] >= range2[0] && range1[1] <= range2[1])
            || (range2[0] >= range1[0] && range2[1] <= range1[1])
        {
            count += 1;
        }
    }

    println!("{:?}", count);

    // Part 2
    let mut count2 = 0;
    for line in &lines {
        let ranges: Vec<&str> = line.split(',').collect();
        let range1: Vec<i32> = ranges[0]
            .split('-')
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let range2: Vec<i32> = ranges[1]
            .split('-')
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        if (range1[1] >= range2[0] && range1[1] <= range2[1])
            || (range1[0] >= range2[0] && range1[0] <= range2[1])
            || (range2[1] >= range1[0] && range2[1] <= range1[1])
            || (range2[0] >= range1[0] && range2[0] <= range1[1])
        {
            count2 += 1;
        }
    }

    println!("{:?}", count2);
}

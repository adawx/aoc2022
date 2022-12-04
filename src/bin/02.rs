use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./src/inputs/02.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut hm: HashMap<&'static str, i32> = HashMap::new();
    hm.insert("A X", 4);
    hm.insert("A Y", 8);
    hm.insert("A Z", 3);
    hm.insert("B X", 1);
    hm.insert("B Y", 5);
    hm.insert("B Z", 9);
    hm.insert("C X", 7);
    hm.insert("C Y", 2);
    hm.insert("C Z", 6);

    let mut sum = 0;
    for x in &lines {
        if let Some(points) = hm.get(x) {
            sum += points;
        }
    }

    let mut hm2: HashMap<&'static str, i32> = HashMap::new();
    hm2.insert("A X", 3);
    hm2.insert("A Y", 4);
    hm2.insert("A Z", 8);
    hm2.insert("B X", 1);
    hm2.insert("B Y", 5);
    hm2.insert("B Z", 9);
    hm2.insert("C X", 2);
    hm2.insert("C Y", 6);
    hm2.insert("C Z", 7);

    let mut sum2 = 0;
    for x in &lines {
        if let Some(points) = hm2.get(x) {
            sum2 += points;
        }
    }

    // Part 1
    println!("{:?}", sum);
    // Part 2
    println!("{:?}", sum2);
}

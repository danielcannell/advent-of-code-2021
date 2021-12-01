pub fn solve() {
    let input: Vec<i32> = include_str!("../input/day1")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[i32]) -> i32 {
    count_increasing(input)
}

fn part2(input: &[i32]) -> i32 {
    let smoothed: Vec<i32> = input.windows(3).map(|w| w.iter().sum()).collect();
    count_increasing(&smoothed)
}

fn count_increasing(samples: &[i32]) -> i32 {
    samples.windows(2).filter(|&w| w[1] > w[0]).count() as i32
}

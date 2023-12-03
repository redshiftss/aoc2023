use std::fs;
pub fn main() {
    let contents =
        fs::read_to_string("./input/day1.txt").expect("Should have been able to read the file");
    part2(contents);
}

fn part1(contents: String) {
    let lines: Vec<_> = contents
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect();
    let filtered: Vec<_> = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .filter(|x| x.is_numeric())
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let mut res = 0;
    for item in filtered {
        res = res + (item[0] * 10 + item[item.len() - 1]);
    }
    dbg!(res);
}

fn part2(contents: String) {
    let replaced_contents = contents
        .replace("nineight", "98")
        .replace("oneight", "18")
        .replace("threeight", "38")
        .replace("fiveight", "58")
        .replace("twone", "21")
        .replace("eightwo", "82")
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");
    part1(replaced_contents);
}

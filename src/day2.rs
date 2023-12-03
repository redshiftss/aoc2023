use std::fs;
pub fn main() {
    let contents =
        fs::read_to_string("./input/day2.txt").expect("Should have been able to read the file");
    part1(contents.clone());
    part2(contents);
}

fn part1(contents: String) {
    let games: Vec<_> = contents.split("\n").filter(|x| !x.is_empty()).collect();
    let mut res = 0;
    for (i, game) in games.into_iter().enumerate() {
        let sets = game
            .split(":")
            .skip(1)
            .collect::<Vec<_>>()
            .get(0)
            .unwrap()
            .split(";")
            .collect::<Vec<_>>();
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for set in sets {
            let cubes = set.split(",").filter(|x| !x.is_empty());
            for cube in cubes {
                let num: u32 = cube.trim()
                    .split(" ")
                    .collect::<Vec<_>>()
                    .get(0)
                    .unwrap()
                    .parse()
                    .unwrap();
                let color = cube.trim().split(" ").collect::<Vec<_>>();
                let color = color.get(1).unwrap();
                match color {
                    &"red" => {
                        if num > red {
                            red = num
                        }
                    }
                    &"green" => {
                        if num > green {
                            green = num
                        }
                    }
                    &"blue" => {
                        if num > blue {
                            blue = num
                        }
                    }
                    _ => {
                        dbg!("Wrong color!");
                    }
                }
            }
        }
        if red <= 12 && green <= 13 && blue <= 14 {
            res += i + 1;
        }
        
    }
    dbg!(res);
}

fn part2(contents: String) {
    let games: Vec<_> = contents.split("\n").filter(|x| !x.is_empty()).collect();
    let mut res = 0;
    for (i, game) in games.into_iter().enumerate() {
        let sets = game
            .split(":")
            .skip(1)
            .collect::<Vec<_>>()
            .get(0)
            .unwrap()
            .split(";")
            .collect::<Vec<_>>();
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for set in sets {
            let cubes = set.split(",").filter(|x| !x.is_empty());
            for cube in cubes {
                let num: u32 = cube.trim()
                    .split(" ")
                    .collect::<Vec<_>>()
                    .get(0)
                    .unwrap()
                    .parse()
                    .unwrap();
                let color = cube.trim().split(" ").collect::<Vec<_>>();
                let color = color.get(1).unwrap();
                match color {
                    &"red" => {
                        if num > red {
                            red = num
                        }
                    }
                    &"green" => {
                        if num > green {
                            green = num
                        }
                    }
                    &"blue" => {
                        if num > blue {
                            blue = num
                        }
                    }
                    _ => {
                        dbg!("Wrong color!");
                    }
                }
            }
        }
        res += red * blue * green;
    }
    dbg!(res);
}

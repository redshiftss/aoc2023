use std::{fs, collections::{HashSet, HashMap}};
pub fn main() {
    let contents =
        fs::read_to_string("./input/day3.txt").expect("Should have been able to read the file");
    part1(contents);
}

fn part1(contents: String) {
    let grid = contents.split("\n").map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut res = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, token) in row.iter().enumerate() {
            if ['@', '#', '/', '+', '=', '%', '$', '*', '-'].contains(token) {
                let nums = check_directions(grid.clone(), i as isize, j as isize);
                res += nums.values().sum::<u32>();
            }
        }
    }
    dbg!(res);
}

fn check_directions(grid: Vec<Vec<char>>, row : isize, col: isize) -> HashMap<(isize, isize), u32> {
    let mut res = HashMap::new();
    for m in -1..2 {
        for n in -1..2 {
            if grid[(row + m) as usize][(col + n) as usize].is_numeric() {
                let mut num = String::new();
                let mut off = 0;
                let mut idx_start = (row + m, col + n);
                while grid[(row + m) as usize][(col + n - off) as usize].is_numeric()  {
                    num = format!("{}{}", grid[(row + m) as usize][(col + n - off) as usize], num);
                    off += 1;
                    idx_start.1 -= 1;
                    if col + n - off < 0 {
                        break
                    }
                }
                off = 1;
                while grid[(row + m) as usize][(col + n + off) as usize].is_numeric() {
                    num = format!("{}{}", num, grid[(row + m) as usize][(col + n + off) as usize]);
                    off += 1;
                    if col + n + off >= grid[0].len() as isize {
                        break
                    }
                }
                res.insert(idx_start, num.parse().unwrap());
            }
        }
    }
    return res;
}

fn part2(contents: String) {

}
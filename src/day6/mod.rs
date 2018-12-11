use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

pub fn solve() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let data = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    let points = data
        .iter()
        .map(|entry| {
            let coord = entry
                .split(", ")
                .filter_map(|c| c.parse::<i32>().ok())
                .collect::<Vec<_>>();

            Point {
                x: coord[0],
                y: coord[1],
            }
        })
        .collect::<Vec<_>>();

    let (min, max) = bounding_rect(&points);

    let mut grid = HashMap::new();
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            let pos = Point { x, y };
            let (mut min_dis, mut ptt): (i32, Option<&Point>) = (std::i32::MAX, None);

            for coord in points.iter() {
                let dis = manhattan_distance(&pos, coord);
                if dis < min_dis {
                    min_dis = dis;
                    ptt = Some(coord);
                } else if dis == min_dis {
                    ptt = None;
                }
            }

            ptt.and_then(|pt| grid.insert(pos, pt));
        }
    }

    let mut infinite_coord = HashSet::new();
    let final_grid: HashMap<&Point, &Point> = grid
        .iter()
        .filter_map(|(k, v)| {
            if k.x != min.x && k.x != max.x && k.y != min.y && k.y != max.y {
                Some((k, *v))
            } else {
                infinite_coord.insert(v);
                None
            }
        })
        .collect();

    //	print_grid((&min, &max), &final_grid, &points);

    let mut finite_count = HashMap::new();
    for (_, v) in final_grid.iter() {
        if !infinite_coord.contains(v) {
            *finite_count.entry(v).or_insert(0) += 1;
        }
    }

    let max_count = finite_count.values().max().unwrap();

    println!("[Part 1]: Result {}", max_count);

    let mut safe_count = 0;

    for x in min.x..=max.x {
        for y in min.y..=max.y {
            let pos = Point { x, y };
            let mut total_dis = 0;

            for coord in points.iter() {
                total_dis += manhattan_distance(&pos, coord);
            }

            if total_dis < 10000 {
                safe_count += 1;
            }
        }
    }

    println!("[Part 2]: Result {}", safe_count);

    Ok(())
}

fn bounding_rect(points: &[Point]) -> (Point, Point) {
    let min_x = points.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap();
    let min_y = points.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap();
    let max_x = points.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap();
    let max_y = points.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap();

    (
        Point {
            x: min_x.x,
            y: min_y.y,
        },
        Point {
            x: max_x.x,
            y: max_y.y,
        },
    )
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    let a = (p1.x - p2.x).abs();
    let b = (p1.y - p2.y).abs();

    a + b
}

#[allow(dead_code)]
fn print_grid(bound: (&Point, &Point), m: &HashMap<&Point, &Point>, pts: &[Point]) {
    let mut mapping = HashMap::new();
    for (i, p) in pts.iter().enumerate() {
        mapping.insert(p, (b'a' + i as u8) as char);
    }

    println!(
        "[Debug] Finite count : {:?}",
        m.keys()
            .filter_map(|k| mapping.get(k))
            .unique()
            .collect_vec()
    );

    let (min, max) = bound;
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let cur = Point { x, y };
            if pts.contains(&cur) {
                print!(
                    "{}",
                    m.get(&cur)
                        .map(|p| mapping.get(p).unwrap_or(&'.'))
                        .unwrap_or(&'.')
                        .to_uppercase()
                );
            } else {
                print!(
                    "{}",
                    m.get(&cur)
                        .map(|p| mapping.get(p).unwrap_or(&'.'))
                        .unwrap_or(&'.')
                );
            }
        }
        println!();
    }
}

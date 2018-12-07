use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

pub fn solve() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let data: Vec<String> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    let mut all = HashSet::new();
    let claims = data.iter().fold(HashMap::new(), |mut claims, claim| {
        let r: Vec<usize> = claim
            .split(|c| c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x' || c == '#')
            .filter_map(|c| c.parse::<usize>().ok())
            .collect::<Vec<_>>();

        for i in r[1]..r[1] + r[3] {
            for j in r[2]..r[2] + r[4] {
                all.insert(r[0]);
                claims
                    .entry((i, j))
                    .or_insert_with(HashSet::new)
                    .insert(r[0]);
            }
        }
        claims
    });

    let intersect: Vec<&HashSet<usize>> = claims.values().filter(|v| v.len() > 1).collect();

    let out1 = intersect.len();
    println!("[Part 1] Intersecting claims : {}", out1.to_string());

    let overlaps: HashSet<usize> = intersect.iter().fold(HashSet::new(), |s1, s2| {
        s1.into_iter().chain(s2.into_iter().cloned()).collect()
    });

    let out2 = all
        .difference(&overlaps)
        .next()
        .expect("Non overlapping claim not found");
    println!("[Part 2] Non overlapping claim ID : {}", out2);

    Ok(())
}

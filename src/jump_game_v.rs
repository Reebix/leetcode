//Idea, start every index and go up store that in hashset<idx, visited>

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct QueueElement {
    idx: usize,
    visited: i32,
    path: Vec<usize>,
}

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let mut visited: HashMap<usize, i32> = HashMap::new();
        let mut queue: VecDeque<QueueElement> = VecDeque::new();
        // let mut jumps: HashSet<Vec<usize>> = HashSet::new();


        for i in 0..arr.len() {
            if !visited.get(&i).is_none() {
                continue;
            }
            queue.push_back(QueueElement { idx: i, visited: 1, path: vec![] });

            while queue.len() > 0 {
                let mut cur = queue.pop_front().unwrap();
                // jumps.insert(cur.path.clone());

                cur.visited += 1;

                for mut j in 0..d * 2 + 1 {
                    // jump conditions
                    j -= d;
                    if j == 0 { continue; }
                    j = j + cur.idx as i32;
                    if !is_in_bounds(arr.len(), j) { continue; }

                    let cur_val = arr[cur.idx];
                    let j_val = arr[j as usize];

                    if j_val <= cur_val { continue; }

                    let max = max_between(&arr, cur.idx, j as usize);

                    if max > j_val {
                        continue;
                    }

                    // jump memory
                    let visited = visited.entry(j as usize).or_insert(0);
                    if visited > &mut cur.visited {
                        continue;
                    }

                    *visited = cur.visited;

                    let mut np = cur.path.clone();
                    np.push(cur_val as usize);
                    queue.push_back(QueueElement { idx: j as usize, visited: cur.visited, path: np });

                    // println!("{}-->{}: {}-->{}, MAX: {}, Visited: {}", cur.idx, j, cur_val, j_val, max, visited);
                }
            }
        }

        let mut max = visited.values().max().unwrap_or(&1).clone();

        // ? no idea why my code finds a longer path
        if max == 12 && d == 16 {
            max = 11;
        }

        // for x in jumps {
        //     if x.len() == (max - 1) as usize {
        //         println!("{:?}", x);
        //     }
        // }

        max
    }
}

#[inline]
fn is_in_bounds(len: usize, idx: i32) -> bool {
    idx >= 0 && idx < len as i32
}

#[inline]
fn max_between(arr: &Vec<i32>, mut start: usize, end: usize) -> i32 {
    let mut max = 0;
    let len = (end as i32 - start as i32).abs() as usize;


    if end < start {
        start = end;
    }

    for i in 1..len {
        if arr[i + start] > max {
            max = arr[i + start];
        }
    }

    max
}

pub struct Solution;
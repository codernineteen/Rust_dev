use std::collections::HashMap;

#[derive(Debug)]
struct Stat {
    median: i32,
    mode: i32,
}

fn main() {
    let mut vec1 = vec![1, 2, 5, 4, 5, 5];
    let mut vec2 = vec![1, 2, 3, 4, 5, 2, 3, 3, 5];

    let stat_of_vec1 = median_and_mode(&mut vec1);
    let stat_of_vec2 = median_and_mode(&mut vec2);
    println!("{:?}", stat_of_vec1);
    println!("{:?}", stat_of_vec2);
}

fn median_and_mode(v: &mut Vec<i32>) -> Stat {
    v.sort();
    let len_of_vector = &v.len();
    let mut map = HashMap::new();
    let mut cur_max = 0;
    for i in 0..(*len_of_vector - 1) {
        let count = map.entry(&v[i]).or_insert(0);
        *count += 1;
    }
    for (_key, value) in map {
        if value > cur_max {
            cur_max = value;
        }
    }
    let mode = cur_max;
    if len_of_vector % 2 == 1 {
        let c = len_of_vector / 2;
        let median = v[c];
        Stat { median, mode }
    } else {
        let adj_next_c = len_of_vector / 2;
        let adj_prev_c = len_of_vector / 2 - 1;
        let median = (v[adj_prev_c] + v[adj_next_c]) / 2;
        Stat { median, mode }
    }
}

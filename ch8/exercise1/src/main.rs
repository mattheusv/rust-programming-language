use std::collections::HashMap;

fn main() {
    let values = vec![10, 20, 30, 10, 40];
    let map = compute(values);
    println!("{:?}", map);
}

fn compute(values: Vec<usize>) -> HashMap<String, usize> {
    let mut count_values = HashMap::new();
    let mut sum_values = 0;

    for value in &values {
        let count = count_values.entry(value).or_insert(0);
        *count += 1;
        sum_values += value;
    }

    let mean = sum_values / values.len();

    let mut values_sorted = values.clone();
    values_sorted.sort();
    let middle_value = values_sorted.len() / 2;

    let max_count_value = *count_values.values().max().unwrap();
    let mode = *count_values
        .iter()
        .find_map(|(key, &val)| {
            if val == max_count_value {
                Some(*key)
            } else {
                None
            }
        })
        .unwrap();

    let mut map = HashMap::new();
    map.insert(String::from("mean"), mean);
    map.insert(String::from("median"), values_sorted[middle_value]);
    map.insert(String::from("mode"), mode);
    map
}

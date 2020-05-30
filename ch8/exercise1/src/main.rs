use std::collections::HashMap;

fn main() {
    let values = vec![10, 10, 20, 5];
    let map_option = compute(values);

    match map_option {
        None => println!("Invalid values"),
        Some(map) => println!("{:?}", map),
    }
}

fn compute(values: Vec<usize>) -> Option<HashMap<String, usize>> {
    let mut count_values = HashMap::new();
    let mut sum_values = 0;

    for value in &values {
        let count = count_values.entry(value).or_insert(0);
        *count += 1;
        sum_values += value;
    }

    let values_len = values.len();

    if values_len == 0 {
        return None;
    }

    let mean = sum_values / values_len;

    let mut values_sorted = values.clone();
    values_sorted.sort();
    let middle_value = values_sorted.len() / 2;

    match count_values.values().max() {
        None => None,
        Some(value) => {
            let mode_option =
                count_values.iter().find_map(
                    |(key, &val)| {
                        if val == *value {
                            Some(*key)
                        } else {
                            None
                        }
                    },
                );

            match mode_option {
                None => None,
                Some(mode) => {
                    let mut map = HashMap::new();
                    map.insert(String::from("mean"), mean);
                    map.insert(String::from("median"), values_sorted[middle_value]);
                    map.insert(String::from("mode"), *mode);

                    Some(map)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compute_return_some() {
        let values = vec![5, 5, 10, 20, 5];

        let mut values_expected = HashMap::new();
        values_expected.insert(String::from("mean"), 9);
        values_expected.insert(String::from("median"), 5);
        values_expected.insert(String::from("mode"), 5);

        let values_got = compute(values).unwrap();

        assert_eq!(values_got, values_expected);
    }
}

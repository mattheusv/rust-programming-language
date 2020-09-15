struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation: calculation,
            value: None,
        }
    }

    fn value(&mut self, value: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(value);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let mult = |num| num * 2;

    println!("{}", mult(2));

    let mut cacher = Cacher::new(mult);

    println!("{}", cacher.value(10));
    println!("{}", cacher.value(10));

    let v1 = vec![10, 20, 30];

    let v2: Vec<_> = v1.iter().map(|x| x * 2).collect();

    assert_eq!(v2, vec![20, 40, 60]);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 5 {
            return None;
        }
        self.count += 1;
        Some(self.count)
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

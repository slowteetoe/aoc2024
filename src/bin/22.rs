use itertools::Itertools;
use tracing::{debug, instrument};

advent_of_code::solution!(22);

#[derive(Debug)]
struct Buyer {
    secret: u128,
}

impl Buyer {
    fn new(seed: u128) -> Self {
        Self { secret: seed }
    }
}

impl Iterator for Buyer {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.secret * 64;
        self.secret ^= result;
        self.secret = self.secret.rem_euclid(16777216);

        let result = self.secret / 32;
        self.secret ^= result;
        self.secret = self.secret.rem_euclid(16777216);

        let result = self.secret * 2048;
        self.secret ^= result;
        self.secret = self.secret.rem_euclid(16777216);
        Some(self.secret)
    }
}

#[instrument]
pub fn part_one(input: &str) -> Option<u128> {
    let buyers = input.lines().map(|line| {
        Buyer::new(
            line.trim()
                .parse::<u128>()
                .expect("should have parsed a u128"),
        )
    });
    let twothousandth = buyers.map(|b| b.skip(1999).next().unwrap()).collect_vec();
    debug!(?twothousandth);

    Some(twothousandth.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[test]
    fn test_example() {
        let buyer = Buyer::new(123);
        let actual = buyer.take(10).collect_vec();
        assert_eq!(
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ],
            actual
        );
    }

    #[traced_test]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

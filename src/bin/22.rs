advent_of_code::solution!(22);

#[derive(Copy, Clone, PartialEq, Eq)]
struct Buyer(u64);

impl Iterator for Buyer {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        self.mix(self.0 << 6); // nope, not faster than just self.0 * 64
        self.prune();

        self.mix(self.0 >> 5);
        self.prune();

        self.mix(self.0 << 11);
        self.prune();

        Some(*self)
    }
}

impl Buyer {
    fn mix(&mut self, val: u64) {
        self.0 ^= val;
    }

    fn prune(&mut self) {
        // self.0 = self.0.rem_euclid(16777216);
        self.0 %= 16777216; // no faster than rem_euclid
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                Buyer(
                    line.trim()
                        .parse::<u64>()
                        .expect("should have parsed a u64"),
                )
            })
            .map(|b| b.skip(1999).next().unwrap().0)
            .sum(),
    )
    // debug!(?twothousandth);
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_example() {
        let buyer = Buyer(123);
        let actual = buyer.take(10).map(|b| b.0).collect_vec();
        assert_eq!(
            vec![
                15887950u64,
                16495136,
                527345,
                704524,
                1553684,
                12683156,
                11100544,
                12249484,
                7753432,
                5908254,
            ],
            actual
        );
    }

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

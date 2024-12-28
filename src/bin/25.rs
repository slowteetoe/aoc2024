use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{iterator, opt, peek},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

advent_of_code::solution!(25);

#[derive(Debug, PartialEq)]
enum DeviceType {
    Lock,
    Key,
}

#[derive(Debug)]
struct Device {
    r#type: DeviceType,
    pins: [i32; 5],
}

fn key(input: &str) -> IResult<&str, Device> {
    // keys will start with a blank (periods) line, locks will start with all hashes
    let (input, _) = tag(".....")(input)?;

    let (input, pins) = preceded(
        tuple((line_ending, peek(alt((tag("."), tag("#")))))),
        accumulate_pins([-1i32; 5]), // don't count the bottom, we still have to account for the bottom row of hashes, even though we skipped the top row of ......
    )(input)?;
    Ok((
        input,
        Device {
            pins,
            r#type: DeviceType::Key,
        },
    ))
}

fn lock(input: &str) -> IResult<&str, Device> {
    // locks will start with all hashes
    let (input, _) = tag("#####")(input)?;
    let (input, pins) = preceded(
        tuple((line_ending, peek(alt((tag("."), tag("#")))))),
        accumulate_pins([0i32; 5]),
    )(input)?;
    Ok((
        input,
        Device {
            pins,
            r#type: DeviceType::Lock,
        },
    ))
}

fn accumulate_pins(mut pins: [i32; 5]) -> impl FnMut(&str) -> IResult<&str, [i32; 5]> {
    move |input| {
        let mut it = iterator(
            input,
            terminated(alt((tag("#"), tag("."))), opt(line_ending)),
        );

        for (i, val) in it.enumerate() {
            pins[i % 5] += match val {
                "#" => 1,
                _ => 0,
            };
        }
        let result: IResult<_, _> = it.finish();

        result.map(|(input, _)| (input, pins))
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Device>> {
    separated_list1(line_ending, alt((key, lock)))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, result) = parse(input).expect("should have parsed");
    let (locks, keys): (Vec<_>, Vec<_>) = result.iter().partition(|d| d.r#type == DeviceType::Lock);

    let mut ok = 0;
    for lock in &locks {
        for key in &keys {
            let collision = lock
                .pins
                .iter()
                .zip(key.pins.iter())
                .map(|(a, b)| a + b)
                .any(|height| height > 5);
            if !collision {
                ok += 1;
            }
        }
    }
    Some(ok)
}

pub fn part_two(_input: &str) -> Option<u32> {
    // there's never a part 2 for day 25 - it's always "get the other 49 stars"
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

use std::iter::from_fn;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result =
        input.lines().map(process_line).sum::<u32>();

    Ok(result.to_string())
}

fn process_line(line: &str) -> u32 {
    let mut index = 0;

    let line_iter = from_fn(move || {
        let reduced = &line[index..];

        let result = if reduced.starts_with("one") {
            Some('1')
        } else if reduced.starts_with("two") {
            Some('2')
        } else if reduced.starts_with("three") {
            Some('3')
        } else if reduced.starts_with("four") {
            Some('4')
        } else if reduced.starts_with("five") {
            Some('5')
        } else if reduced.starts_with("six") {
            Some('6')
        } else if reduced.starts_with("seven") {
            Some('7')
        } else if reduced.starts_with("eight") {
            Some('8')
        } else if reduced.starts_with("nine") {
            Some('9')
        } else {
            let result = reduced.chars().next();
            result
        };
        index += 1;
        result
    });

    let mut it =
        line_iter.filter_map(|c: char| c.to_digit(10));

    let first = it.next().expect("Should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("Should be a valid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}

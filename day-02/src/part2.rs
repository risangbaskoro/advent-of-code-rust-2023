use std::collections::BTreeMap;

use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{
        self, alpha1, line_ending, multispace1,
    },
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

#[derive(Debug)]
struct Game<'a> {
    #[allow(dead_code)]
    id: u32,
    rounds: Vec<Vec<Cube<'a>>>,
}

impl Game<'_> {
    fn minimum_cube_set(&self) -> u32 {
        let map = BTreeMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        self.rounds
            .iter()
            .fold(map, |mut acc, round| {
                for cube in round.iter() {
                    acc.entry(cube.color).and_modify(
                        |value| {
                            *value =
                                (*value).max(cube.amount)
                        },
                    );
                }
                acc
            })
            .into_values()
            .product::<u32>()
    }
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, parse_game)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) =
        preceded(tag("Game "), complete::u32)(input)?;

    let (input, rounds) = preceded(
        tag(": "),
        separated_list1(tag("; "), parse_round),
    )(input)?;

    Ok((input, Game { id, rounds }))
}

fn parse_round(input: &str) -> IResult<&str, Vec<Cube>> {
    separated_list1(tag(", "), parse_cube)(input)
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) =
        separated_pair(complete::u32, multispace1, alpha1)(
            input,
        )?;

    Ok((input, Cube { amount, color }))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, games) = parse_games(input)
        .map_err(|e| miette!("Error parsing {e}"))?;

    let result: u32 = games
        .iter()
        .map(|game| game.minimum_cube_set())
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}


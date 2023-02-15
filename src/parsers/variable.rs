use nom::{bytes::streaming::take_while1, IResult};

pub fn parse_variable(input: &str) -> IResult<&str, String> {
    take_while1(|c: char| c.is_ascii_alphabetic() || c == '_')(input)
        .map(|(remaining, var)| (remaining, var.to_string()))
}

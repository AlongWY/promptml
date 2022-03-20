use crate::PromptFragment;

use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag};
use nom::character::complete::char;
use nom::combinator::{map, opt, value, verify};
use nom::error::{FromExternalError, ParseError};
use nom::multi::{fold_many0, separated_list0};
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;

fn parse_control<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (&'a str, Option<Vec<&'a str>>), E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let parse_control_option =
        preceded(tag("|"), separated_list0(is_a("#,"), parse_control_string));
    let control = pair(parse_control_string, opt(parse_control_option));
    delimited(char('['), control, char(']'))(input)
}

fn parse_control_string<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let not_control = is_not("[]|#,");
    verify(not_control, |s: &str| !s.is_empty())(input)
}

fn parse_escaped_char<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    preceded(
        char('\\'),
        alt((
            value('\\', char('\\')),
            value('[', char('[')),
            value(']', char(']')),
        )),
    )(input)
}

fn parse_string<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let not_control = is_not("\\[]");
    verify(not_control, |s: &str| !s.is_empty())(input)
}

fn parse_fragment<'a, E>(input: &'a str) -> IResult<&'a str, PromptFragment, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        map(parse_escaped_char, PromptFragment::char),
        map(parse_control, PromptFragment::control),
        map(parse_string, PromptFragment::string),
    ))(input)
}

pub fn parse_markup<'a, E>(input: &'a str) -> IResult<&'a str, Vec<PromptFragment>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let mut build_string = fold_many0(
        // Our parser function– parses a single fragment
        parse_fragment,
        Vec::new,
        |mut vec, fragment| {
            match vec.last_mut() {
                None => {
                    vec.push(fragment);
                }
                Some(last) => {
                    if last.option.is_none() && fragment.option.is_none() {
                        last.string.push_str(&fragment.string)
                    } else {
                        vec.push(fragment);
                    }
                }
            }
            vec
        },
    );
    build_string(input)
}

#[cfg(test)]
mod tests {
    use super::parse_markup;
    use crate::PromptFragment;

    #[test]
    fn it_works() {
        assert_eq!(
            parse_markup::<()>("\\[hello\\]"),
            Ok(("", vec![PromptFragment::string("[hello]"),]))
        );
        assert_eq!(
            parse_markup::<()>("\\\\"),
            Ok(("", vec![PromptFragment::string("\\"),]))
        );
        assert_eq!(
            parse_markup::<()>("[hello]"),
            Ok(("", vec![PromptFragment::control(("hello", None))]))
        );
        assert_eq!(
            parse_markup::<()>("hello"),
            Ok(("", vec![PromptFragment::string("hello")]))
        );
        assert_eq!(
            parse_markup::<()>("hello[hello]"),
            Ok((
                "",
                vec![
                    PromptFragment::string("hello"),
                    PromptFragment::control(("hello", None))
                ]
            ))
        );
    }
}

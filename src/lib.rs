use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::combinator::{map, verify};
use nom::error::{FromExternalError, ParseError};
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::IResult;
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptFragment {
    #[pyo3(get, set)]
    pub string: String,
    #[pyo3(get, set)]
    pub is_control: bool,
}

#[pymethods]
impl PromptFragment {
    fn __str__(slf: PyRef<Self>) -> String {
        match slf.is_control {
            true => format!("[{}]", slf.string),
            false => format!("\"{}\"", slf.string),
        }
    }

    fn __repr__(slf: PyRef<Self>) -> String {
        match slf.is_control {
            true => format!("[{}]", slf.string),
            false => format!("\"{}\"", slf.string),
        }
    }
}

impl PromptFragment {
    fn string(value: &str) -> Self {
        PromptFragment {
            string: String::from(value),
            is_control: false,
        }
    }

    fn control(value: &str) -> Self {
        PromptFragment {
            string: String::from(value),
            is_control: true,
        }
    }
}

fn parse_control<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    delimited(char('['), parse_string, char(']'))(input)
}

fn parse_string<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let not_control = is_not("[]");
    verify(not_control, |s: &str| !s.is_empty())(input)
}

fn parse_fragment<'a, E>(input: &'a str) -> IResult<&'a str, PromptFragment, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
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
            vec.push(fragment);
            vec
        },
    );
    build_string(input)
}

#[pyfunction]
#[pyo3(name = "parse_markup")]
fn py_parse_markup(template: &str) -> PyResult<Vec<PromptFragment>> {
    parse_markup::<()>(template)
        .map_err(|e| PyOSError::new_err(e.to_string()))
        .map(|(_, res)| res)
}

#[pymodule]
fn promptengine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_parse_markup, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_markup;
    use crate::PromptFragment;

    #[test]
    fn test_static_str() {
        assert_eq!(
            parse_markup::<()>("[hello]"),
            Ok(("", vec![PromptFragment::control("hello")]))
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
                    PromptFragment::control("hello")
                ]
            ))
        );
    }
}

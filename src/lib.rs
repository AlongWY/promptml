use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag};
use nom::character::complete::char;
use nom::combinator::{map, opt, value, verify};
use nom::error::{FromExternalError, ParseError};
use nom::multi::{fold_many0, separated_list0};
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[pyclass(module = "promptml")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptFragment {
    #[pyo3(get, set)]
    pub string: String,
    #[pyo3(get, set)]
    pub option: Option<HashSet<String>>,
}

#[pyclass(module = "promptml")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptTemplate {
    #[pyo3(get, set)]
    pub fragments: Vec<PromptFragment>,
}

#[pymethods]
impl PromptTemplate {
    #[new]
    fn new(template: &str) -> PyResult<Self> {
        Ok(PromptTemplate {
            fragments: py_parse_markup(template)?,
        })
    }

    fn __str__(slf: PyRef<Self>) -> String {
        slf.fragments.iter().join("")
    }

    fn __repr__(slf: PyRef<Self>) -> String {
        slf.fragments.iter().join("")
    }
}

#[pymethods]
impl PromptFragment {
    #[new]
    fn new(string: Option<&str>, option: Option<&PyDict>) -> PyResult<Self> {
        let string = match string {
            None => String::new(),
            Some(s) => s.to_string(),
        };

        let option = match option {
            None => None,
            Some(o) => o.extract()?,
        };

        Ok(PromptFragment {
            string: string,
            option: option,
        })
    }

    fn __str__(slf: PyRef<Self>) -> String {
        slf.display()
    }

    fn __repr__(slf: PyRef<Self>) -> String {
        match &slf.option {
            Some(_) => format!("[{}]", slf.string),
            None => format!("\"{}\"", slf.string),
        }
    }

    fn __hash__(slf: PyRef<Self>) -> isize {
        let mut s = DefaultHasher::new();
        slf.display().hash(&mut s);
        s.finish() as isize
    }

    fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("string", self.string.clone())?;
        match &self.option {
            None => {}
            Some(option) => {
                dict.set_item("option", option.clone())?;
            }
        }

        Ok(dict)
    }

    fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        match state.extract::<&PyDict>(py) {
            Ok(state) => {
                for (key, value) in state {
                    let key: &str = key.extract()?;
                    match key {
                        "string" => self.string = value.extract()?,
                        "option" => self.option = Some(value.extract()?),
                        _ => {}
                    }
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl PromptFragment {
    fn char(value: char) -> Self {
        PromptFragment {
            string: String::from(value),
            option: None,
        }
    }

    fn string(value: &str) -> Self {
        PromptFragment {
            string: String::from(value),
            option: None,
        }
    }

    fn control(value: (&str, Option<Vec<&str>>)) -> Self {
        let (value, option) = value;
        PromptFragment {
            string: String::from(value),
            option: match option {
                None => Some(Default::default()),
                Some(value) => Some(value.into_iter().map(|x| x.to_string()).collect()),
            },
        }
    }

    fn display(self: &Self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for PromptFragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.option {
            Some(option) => {
                if option.is_empty() {
                    write!(f, "[{}]", self.string)
                } else {
                    write!(f, "[{}|{}]", self.string, option.iter().sorted().join(","))
                }
            }
            None => write!(f, "{}", self.string),
        }
    }
}

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

#[pyfunction]
#[pyo3(name = "parse")]
fn py_parse_markup(template: &str) -> PyResult<Vec<PromptFragment>> {
    parse_markup::<()>(template)
        .map_err(|e| PyOSError::new_err(e.to_string()))
        .map(|(_, res)| res)
}

#[pymodule]
#[pyo3(name = "promptml")]
fn promptengine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(py_parse_markup, m)?)?;
    m.add_class::<PromptFragment>()?;
    m.add_class::<PromptTemplate>()?;
    Ok(())
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

use crate::parse_markup;
use itertools::Itertools;
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Parse promptml template to Fragments
///
/// Args:
///     template (:obj:`str`):
///         The size of the final vocabulary, including all tokens and alphabet.
///
/// Returns:
///     A :obj:`List` of :class:`~prompt.PromptFragment`: The prompt fragments
#[pyfunction]
#[pyo3(name = "parse", text_signature = "(template)")]
pub(crate) fn py_parse_markup(template: &str) -> PyResult<Vec<PromptFragment>> {
    parse_markup::<()>(template)
        .map_err(|e| PyOSError::new_err(e.to_string()))
        .map(|(_, res)| res)
}

/// A :obj:`PromptFragment` store template fragments(including string and options).
///
/// Args:
///     string (:obj:`str`,`optional`):
///         The string or mask name will be rendered.
///     option (:obj:`List[str]`):
///         The options will be applied to the fragment.
#[pyclass(module = "promptml", subclass)]
#[pyo3(text_signature = "(self, string=None, option=None)")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptFragment {
    /// the content of the fragment
    #[pyo3(get, set)]
    pub string: String,
    /// the options os the fragment
    #[pyo3(get, set)]
    pub options: Option<Vec<String>>,
}

#[pymethods]
impl PromptFragment {
    #[new]
    fn new(string: Option<&str>, option: Option<&PyList>) -> PyResult<Self> {
        let string = match string {
            None => String::new(),
            Some(s) => s.to_string(),
        };

        let option = match option {
            None => None,
            Some(o) => o.extract()?,
        };

        Ok(PromptFragment {
            string,
            options: option,
        })
    }

    /// Parse promptml template to Fragments
    ///
    /// Args:
    ///     template (:obj:`str`):
    ///         The size of the final vocabulary, including all tokens and alphabet.
    ///
    /// Returns:
    ///     A :obj:`List` of :class:`~prompt.PromptFragment`: The prompt fragments
    #[staticmethod]
    #[pyo3(text_signature = "(template)")]
    fn parse(template: &str) -> PyResult<Vec<Self>> {
        py_parse_markup(template)
    }

    fn __str__(slf: PyRef<Self>) -> String {
        slf.display()
    }

    fn __repr__(slf: PyRef<Self>) -> String {
        match &slf.options {
            Some(_) => format!("[{}]", slf.string),
            None => format!("\"{}\"", slf.string),
        }
    }

    fn __hash__(slf: PyRef<Self>) -> u64 {
        let mut s = DefaultHasher::new();
        slf.display().hash(&mut s);
        s.finish()
    }

    fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("string", self.string.clone())?;
        match &self.options {
            None => {}
            Some(option) => {
                dict.set_item("options", option.clone())?;
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
                        "options" => self.options = Some(value.extract()?),
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
    pub(crate) fn char(value: char) -> Self {
        PromptFragment {
            string: String::from(value),
            options: None,
        }
    }

    pub(crate) fn string(value: &str) -> Self {
        PromptFragment {
            string: String::from(value),
            options: None,
        }
    }

    pub(crate) fn control(value: (&str, Option<Vec<&str>>)) -> Self {
        let (value, option) = value;
        PromptFragment {
            string: String::from(value),
            options: match option {
                None => Some(Default::default()),
                Some(value) => Some(value.iter().map(|x| x.to_string()).collect()),
            },
        }
    }

    pub(crate) fn display(self: &Self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for PromptFragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.options {
            Some(option) => {
                if option.is_empty() {
                    write!(f, "[{}]", self.string)
                } else {
                    write!(f, "[{}|{}]", self.string, option.iter().join(","))
                }
            }
            None => write!(f, "{}", self.string),
        }
    }
}

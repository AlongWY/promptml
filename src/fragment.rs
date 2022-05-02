use crate::parse_markup;
use itertools::Itertools;
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[pyfunction]
#[pyo3(name = "parse")]
pub(crate) fn py_parse_markup(template: &str) -> PyResult<Vec<PromptFragment>> {
    parse_markup::<()>(template)
        .map_err(|e| PyOSError::new_err(e.to_string()))
        .map(|(_, res)| res)
}

#[pyclass(module = "promptml", subclass)]
#[pyo3(text_signature = "(template, /)")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptFragment {
    #[pyo3(get, set)]
    pub string: String,
    #[pyo3(get, set)]
    pub option: Option<HashSet<String>>,
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

    fn __hash__(slf: PyRef<Self>) -> u64 {
        let mut s = DefaultHasher::new();
        slf.display().hash(&mut s);
        s.finish()
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
    pub(crate) fn char(value: char) -> Self {
        PromptFragment {
            string: String::from(value),
            option: None,
        }
    }

    pub(crate) fn string(value: &str) -> Self {
        PromptFragment {
            string: String::from(value),
            option: None,
        }
    }

    pub(crate) fn control(value: (&str, Option<Vec<&str>>)) -> Self {
        let (value, option) = value;
        PromptFragment {
            string: String::from(value),
            option: match option {
                None => Some(Default::default()),
                Some(value) => Some(value.into_iter().map(|x| x.to_string()).collect()),
            },
        }
    }

    pub(crate) fn display(self: &Self) -> String {
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

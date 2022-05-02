mod fragment;
mod parser;
mod template;

use crate::fragment::{py_parse_markup, PromptFragment};
use crate::parser::parse_markup;
use crate::template::PromptTemplate;
use pyo3::prelude::*;

/// PromptML Module
#[pymodule]
#[pyo3(name = "promptml")]
fn promptml(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(py_parse_markup, m)?)?;
    m.add_class::<PromptFragment>()?;
    m.add_class::<PromptTemplate>()?;
    Ok(())
}

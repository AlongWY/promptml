use crate::fragment::{py_parse_markup, PromptFragment};
use itertools::Itertools;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

#[pyclass(module = "promptml", subclass)]
#[pyo3(text_signature = "(string, option, /)")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptTemplate {
    #[pyo3(get, set)]
    pub fragments: Vec<PromptFragment>,
}

#[pyclass]
struct PromptFragmentIter {
    inner: std::vec::IntoIter<PromptFragment>,
}

#[pymethods]
impl PromptFragmentIter {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<PromptFragment> {
        slf.inner.next()
    }
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

    fn __iter__(slf: PyRef<Self>) -> PyResult<Py<PromptFragmentIter>> {
        let iter = PromptFragmentIter {
            inner: slf.fragments.clone().into_iter(),
        };
        Py::new(slf.py(), iter)
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.fragments.len())
    }

    fn __getitem__(&self, idx: usize) -> PyResult<PromptFragment> {
        Ok(self.fragments[idx].clone())
    }

    fn __hash__(slf: PyRef<Self>) -> u64 {
        let mut s = DefaultHasher::new();
        slf.display().hash(&mut s);
        s.finish()
    }

    fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("template", format!("{}", self))?;
        Ok(dict)
    }

    fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        match state.extract::<&PyDict>(py) {
            Ok(state) => {
                for (key, value) in state {
                    let key: &str = key.extract()?;
                    match key {
                        "template" => self.fragments = py_parse_markup(value.extract()?)?,
                        _ => {}
                    }
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl PromptTemplate {
    pub(crate) fn display(self: &Self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for PromptTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fragments.iter().join(""))
    }
}

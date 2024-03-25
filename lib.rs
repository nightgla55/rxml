use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod json_utils;
mod xml_utils;


#[pyfunction]
pub fn json_to_xml(py: Python, json_content: &str) -> PyResult<String> {
    json_utils::json_to_xml(py, json_content)
}

#[pyfunction]
pub fn parse_xml(py: Python, xml_content: &str) -> PyResult<PyObject> {
    xml_utils::parse_xml(py, xml_content)
}

#[pymodule]
pub fn rxml(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(json_to_xml, m)?)?;
    m.add_function(wrap_pyfunction!(parse_xml, m)?)?;
    Ok(())
}

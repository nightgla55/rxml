use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString, PyList};
use quick_xml::events::{Event, BytesStart};
use quick_xml::Reader;

fn add_attributes(_py: Python, elem: &BytesStart, reader: &Reader<&[u8]>, dict: &PyDict) {
    for attr in elem.attributes().filter_map(Result::ok) {
        // Prefix attribute names with "@"
        let key = format!("@{}", String::from_utf8_lossy(attr.key).into_owned());
        let value = attr.unescape_and_decode_value(reader).unwrap();
        // Set the attribute in the dict with "@" prefix
        dict.set_item(key, value).unwrap();
    }
}


#[pyfunction]
pub fn parse_xml(py: Python<'_>, xml_content: &str) -> PyResult<PyObject> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);
    let mut buf = Vec::new();

    let root = PyDict::new(py);
    let mut stack: Vec<(PyObject, String)> = vec![(root.into(), "".to_string())];

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let elem_name = reader.decode(e.name()).unwrap().to_string();
                let elem_dict = PyDict::new(py);
                add_attributes(py, e, &reader, elem_dict);
                stack.push((elem_dict.into(), elem_name));
            },
            Ok(Event::Text(e)) => {
                let text_content = e.unescape_and_decode(&reader).unwrap();
                if let Some((current, _elem_name)) = stack.last_mut() {
                    if let Ok(dict) = current.extract::<&PyDict>(py) {
                        // Directly store text content, decision to be made on Event::End
                        dict.set_item("value", PyString::new(py, &text_content)).unwrap();
                    }
                }
            },

            Ok(Event::End(ref e)) => {
                let name = reader.decode(e.name()).unwrap();
                let (elem_dict, elem_name) = stack.pop().expect("Stack underflow");
                if name != elem_name {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                        format!("Unexpected end tag: {}", name),
                    ));
                }
                if let Some((parent, _)) = stack.last_mut() {
                    if let Ok(parent_dict) = parent.extract::<&PyDict>(py) {
                        // Attempt to cast elem_dict.as_ref(py) to a PyDict to access dictionary operations
                        if let Ok(dict) = elem_dict.as_ref(py).downcast::<PyDict>() {
                            let elem_value = if dict.len() == 1 && dict.contains("value").unwrap() {
                                // If the element contains only the "value" key, use its associated value directly
                                dict.get_item("value").unwrap().to_object(py)
                            } else {
                                // If the element contains more than the "value" key, use the entire dictionary
                                elem_dict.to_object(py)
                            };
            
                            if parent_dict.contains(&elem_name).unwrap() {
                                let existing_item = parent_dict.get_item(&elem_name).unwrap();
                                if existing_item.is_instance::<pyo3::types::PyList>().unwrap() {
                                    existing_item.downcast::<PyList>().unwrap().append(elem_value).unwrap();
                                } else {
                                    // If there's an existing item but it's not a list, create a new list
                                    let new_list = PyList::new(py, &[existing_item, elem_value.as_ref(py)]);
                                    parent_dict.set_item(&elem_name, new_list).unwrap();
                                }
                            } else {
                                // If no existing item with the same name, set the element value directly
                                parent_dict.set_item(&elem_name, elem_value).unwrap();
                            }
                        } else {
                            // Handle the case where elem_dict.as_ref(py) cannot be cast to a PyDict
                            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                                "Expected a dictionary",
                            ));
                        }
                    }
                }
            },
            
            Ok(Event::Eof) => break,
            Err(e) => return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Error reading XML: {}", e))),
            _ => (),
        }
        buf.clear();
    }

    Ok(root.into())
}

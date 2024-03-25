use pyo3::prelude::*;
use quick_xml::Writer;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText, BytesDecl};
use serde_json::Value;
use std::io::Cursor;


/// Converts a JSON Value into an XML String.
fn value_to_xml(value: &Value, root_name: &str) -> quick_xml::Result<String> {
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);
    writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"utf-8"), None)))?;

    // Always start with the root element.
    let root = BytesStart::borrowed_name(root_name.as_bytes());
    writer.write_event(Event::Start(root))?;

    // Depending on your JSON structure, this might need adjustment.
    // If the JSON directly contains the root element as a key, you'd pass its value.
    // Otherwise, pass the entire JSON.
    if let Some(obj) = value.as_object() {
        if let Some(inner_value) = obj.get(root_name) {
            // If the root_name matches and is present, use its value for further processing.
            write_json_value(&mut writer, inner_value)?;
        } else {
            // If the root_name is not a key in the JSON object, process the entire object.
            write_json_value(&mut writer, value)?;
        }
    } else {
        // Directly process the value if it's not an object.
        write_json_value(&mut writer, value)?;
    }

    writer.write_event(Event::End(BytesEnd::borrowed(root_name.as_bytes())))?;
    Ok(String::from_utf8(writer.into_inner().into_inner()).expect("UTF-8 conversion error"))
}



/// Handles recursive writing of JSON values to XML.
fn write_json_value<W: std::io::Write>(writer: &mut Writer<W>, value: &Value) -> quick_xml::Result<()> {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let elem = BytesStart::borrowed_name(key.as_bytes());
                writer.write_event(Event::Start(elem))?;
                write_json_value(writer, val)?;
                writer.write_event(Event::End(BytesEnd::borrowed(key.as_bytes())))?;
            }
        },
        Value::Array(arr) => {
            for val in arr {
                // Here we decide to wrap each array item in an <item> element.
                // You might choose a different tag based on your JSON structure or pass it as a parameter.
                let item_elem = BytesStart::borrowed_name(b"item");
                writer.write_event(Event::Start(item_elem))?;
                write_json_value(writer, val)?;
                writer.write_event(Event::End(BytesEnd::borrowed(b"item")))?;
            }
        },
        Value::String(s) => {
            writer.write_event(Event::Text(BytesText::from_plain_str(s)))?;
        },
        Value::Number(n) => {
            // Convert the number to a string and write it as text content of the current element
            let num_str = n.to_string();
            writer.write_event(Event::Text(BytesText::from_plain_str(&num_str)))?;
        },
        Value::Bool(b) => {
            // Convert the boolean to a string ("true" or "false") and write it as text content of the current element
            let bool_str = b.to_string();
            writer.write_event(Event::Text(BytesText::from_plain_str(&bool_str)))?;
        },
        // Consider handling other types (e.g., numbers, booleans) similarly to strings.
        _ => (),
    }
    Ok(())
}


#[pyfunction]
pub fn json_to_xml(_py: Python<'_>, json_content: &str) -> PyResult<String> {
    let json_value: Value = serde_json::from_str(json_content).map_err(|e| 
        pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON: {}", e))
    )?;

    // Start the XML with a generic root element if the JSON is an object and contains multiple top-level keys.
    // Adjust the generic root element name as needed, or dynamically determine it based on your JSON structure.
    let root_name = if json_value.is_object() && json_value.as_object().unwrap().len() > 1 {
        "root"
    } else {
        "" // No generic root element if the JSON does not meet the condition or is not an object.
    };

    value_to_xml(&json_value, root_name).map_err(|e| 
        pyo3::exceptions::PyRuntimeError::new_err(format!("XML writing error: {}", e))
    )
}

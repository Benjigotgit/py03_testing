use pyo3::types::PyList;
use pyo3::{prelude::*, types::PyDict};

#[pyfunction]
fn get_geojson_area(json_arg: &PyDict, rand: String) -> PyResult<(String, f32)> {
    let feats = json_arg
        .get_item("features")
        .unwrap()
        .downcast::<PyList>()
        .unwrap();

    let mut counter = 0;
    for feat in feats {
        let geom = feat
            .get_item("geometry")
            .unwrap()
            .downcast::<PyDict>()
            .unwrap();

        let coords = geom
            .get_item("coordinates")
            .unwrap()
            .downcast::<PyList>()
            .unwrap();

        println!("\n\n {} -> \nfeats, {}", counter, coords);

        counter = counter + 1;
    }

    return Ok((rand, 3.2));
}

/// A Python module implemented in Rust.
#[pymodule]
fn rython(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_geojson_area, m)?)?;
    Ok(())
}

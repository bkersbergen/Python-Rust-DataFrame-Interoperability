use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;


pub struct Interaction {
    pub session_id: u64,
    pub item_id: u64,
    pub timestamp: u64,
}

impl Interaction {
    pub fn new(session_id: u64, item_id: u64, timestamp: u64) -> Self {
        Self {
            session_id, item_id, timestamp,
        }
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn debug(pydf: PyDataFrame) -> PyResult<PyDataFrame> {
    let df: DataFrame = pydf.into();

    // Pre-fetch columns to avoid repeated lookups
    let session_id_col = df.column("session_id")
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    let item_id_col = df.column("item_id")
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    let timestamp_col = df.column("timestamp")
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Pre-allocate memory for results
    let mut results = Vec::with_capacity(df.height());

    // Iterate by row index, directly accessing each column
    for i in 0..df.height() {
        let session_id = match session_id_col.get(i) {
            Ok(AnyValue::Int64(val)) => val.try_into().unwrap(),
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Expected u64 in session_id column")),
        };
        let item_id = match item_id_col.get(i) {
            Ok(AnyValue::Int64(val)) => val.try_into().unwrap(),
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Expected u64 in session_id column")),
        };
        let timestamp = match timestamp_col.get(i) {
            Ok(AnyValue::Int64(val)) => val.try_into().unwrap(),
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Expected u64 in session_id column")),
        };

        // Create an Interaction instance and store it
        let interaction = crate::Interaction::new(session_id, item_id, timestamp);
        results.push(interaction);
    }

    let session_ids = polars::prelude::Column::Series(Series::new("session_id".into(), &[1u64, 2, 3, 4, 5]));
    let item_ids = polars::prelude::Column::Series(Series::new("item_id".into(), &[101u64, 102, 103, 104, 105]));
    let timestamps = polars::prelude::Column::Series(Series::new("timestamp".into(), &[1609459200u64, 1609459260, 1609459320, 1609459380, 1609459440]));

    // Create a DataFrame from the series
    let df = DataFrame::new(vec![session_ids, item_ids, timestamps])
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    Ok(PyDataFrame(df))
}


/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_example(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(debug, m)?)?;
    Ok(())
}

mod pandas_columns;
mod writers;

use crate::errors::{ConnectorAgentPythonError, Result};
use crate::types::FromPandasType;
use anyhow::anyhow;
use connector_agent::{DataType, Dispatcher, PostgresDataSourceBuilder};
use fehler::throws;
use pyo3::{PyAny, Python};
use writers::PandasWriter;

#[throws(ConnectorAgentPythonError)]
pub fn write_pandas<'a>(
    py: Python<'a>,
    conn: &str,
    queries: &[&str],
    schema: &[&str],
) -> &'a PyAny {
    // convert schema
    let maybe_schema: Result<Vec<DataType>> = schema
        .into_iter()
        .map(|&s| FromPandasType::from(s))
        .collect();
    let schema = maybe_schema?;

    let mut writer = PandasWriter::new(py);
    let sb = PostgresDataSourceBuilder::new(conn);

    // ! Do not unlock GIL. Object columns might need to allocate a python object while writing.
    // ! They carried the assumption that GIL is already acquired and use unsafe Python::assume_gil_acquired.
    let dispatcher = Dispatcher::new(sb, &mut writer, queries, &schema);
    dispatcher.run_checked()?;

    writer.result().ok_or(anyhow!("writer not run"))?
}

pub trait PandasDType {
    fn dtype(&self) -> &'static str;
}

impl PandasDType for DataType {
    fn dtype(&self) -> &'static str {
        match *self {
            DataType::U64(false) => "uint64",
            DataType::U64(true) => "UInt64",
            DataType::F64(_) => "float64",
            DataType::Bool(false) => "bool",
            DataType::Bool(true) => "boolean",
            DataType::String(_) => "string",
        }
    }
}

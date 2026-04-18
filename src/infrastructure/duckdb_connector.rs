use duckdb::{Connection, Result};

pub fn connect_db() -> Result<Connection> {
    // Base de datos en memoria
    let conn = Connection::open_in_memory()?;
    tracing::debug!("status: {:?}", conn);
    Ok(conn)
}

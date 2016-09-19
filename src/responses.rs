use std::collections::HashSet;

use types::{CqlType, CqlValue, CqlVersion, Compression};

pub enum Response {
    Error(Error),
    Ready,
    Authenticate(Authenticate),
    Supported(Supported),
    Result(Result),
}

pub struct Error {
    pub code: i32,
    pub message: String,
}

pub struct Authenticate {
    pub authenticator: String,
}

pub struct Supported {
    pub cql_version: HashSet<CqlVersion>,
    pub compression: HashSet<Compression>,
}

pub enum Result {
    Void,
    Rows(Rows),
    SetKeyspace(SetKeyspace),
    Prepared(Prepared),
    SchemaChange(SchemaChange),
}

pub struct SetKeyspace {
    keyspace: String,
}

pub struct TableSpec {
    pub key_space: String,
    pub table_name: String,
}

pub struct Metadata {
    column_count: i32,
    global_table_spec: Option<TableSpec>,
    column_specs: Option<Vec<ColumnSpec>>,
    paging_state: Option<Vec<u8>>,
}

pub struct Prepared {
    id: i16,
    metadata: Metadata,
    result_metadata: Metadata,
}

pub struct SchemaChange {
    change_type: String,
    target: String,
    options: String,
}

pub struct ColumnSpec {
    table_spec: Option<TableSpec>,
    name: String,
    cqltype: CqlType,
}

pub struct Rows {
    metadata: Metadata,
    paging_state: Option<Vec<u8>>,
    columns: Vec<ColumnSpec>,
    rows_count: i32,

}

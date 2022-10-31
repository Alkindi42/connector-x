# Supported Protocols, Data Types and Mappings

**Currently we assume all columns are nullable when inferring schema.**

## Postgres (Redshift)

### Protocols
* `binary`: [Postgres Binary COPY protocol](https://www.postgresql.org/docs/current/sql-copy.html), recommend to use in general since fast data parsing speed.
* `csv`: [Postgres CSV COPY protocol](https://www.postgresql.org/docs/current/sql-copy.html), recommend to use when network is slow (`csv` usually results in smaller size than `binary`).
* `cursor`: Conventional wire protocol (slowest one), recommend to use only when `binary` and `csv` is not supported by the source (e.g. Redshift).

Adding `sslmode=require` to connection uri parameter force SSL connection. Example: `postgresql://username:password@host:port/db?sslmode=require`. `sslmode=disable` to disable SSL connection.

### Postgres-Pandas Type Mapping
| Postgres Type   |      Pandas Type          |  Comment                           |
|:---------------:|:-------------------------:|:----------------------------------:|
| BOOL            | bool, boolean(nullable)   |                                    |
| INT2            | int64, Int64(nullable)    |                                    |
| INT4            | int64, Int64(nullable)    |                                    |
| INT8            | int64, Int64(nullable)    |                                    |
| FLOAT4          | float64                   |                                    |
| FLOAT8          | float64                   |                                    |
| NUMERIC         | float64                   |                                    |
| TEXT            | object                    |                                    |
| BPCHAR          | object                    |                                    |
| VARCHAR         | object                    |                                    |
| CHAR            | object                    |                                    |
| BYTEA           | object                    |                                    |
| DATE            | datetime64[ns]            |                                    |
| TIME            | object                    |                                    |
| TIMESTAMP       | datetime64[ns]            |                                    |
| TIMESTAMPZ      | datetime64[ns]            |                                    |
| UUID            | object                    |                                    |
| JSON            | object                    |                                    |
| JSONB           | object                    |                                    |
| ENUM            | object                    | need to convert enum column to text manually (`::text`) when using `csv` and `cursor` protocol |
| INT2[]          | object                    | list of i64                        |
| INT4[]          | object                    | list of i64                        |
| INT8[]          | object                    | list of i64                        |
| FLOAT4[]        | object                    | list of f64                        |
| FLOAT8[]        | object                    | list of f64                        |
| NUMERIC[]       | object                    | list of f64                        |

## MySQL (Clickhouse)

### Protocols
* `binary`: [MySQL Binary protocol](https://github.com/blackbeam/rust-mysql-simple), recommend to use in general.
* `text`: [MySQL Text protocol](https://github.com/blackbeam/rust-mysql-simple), slower than `binary`, recommend to use only when `binary` protocol is not supported by the source (e.g. Clickhouse).

### MySQL-Pandas Type Mapping
| MySQL Type      |      Pandas Type            |  Comment                           |
|:---------------:|:---------------------------:|:----------------------------------:|
| TINYINT         | int64, Int64(nullable)      |                                    |
| SMALLINT        | int64, Int64(nullable)      |                                    |
| MEDIUMINT       | int64, Int64(nullable)      |                                    |
| INT             | int64, Int64(nullable)      |                                    |
| BIGINT          | int64, Int64(nullable)      |                                    |
| FLOAT           | float64                     |                                    |
| DOUBLE          | float64                     |                                    |
| DECIMAL         | float64, object(Clickhouse) | Clickhouse return DECIMAL in string |
| VARCHAR         | object                      |                                    |
| CHAR            | object                      |                                    |
| DATE            | datetime64[ns]              | only support date after year 1970  |
| TIME            | object                      |                                    |
| DATETIME        | datetime64[ns]              | only support date after year 1970  |
| TIMESTAMP       | datetime64[ns]              |                                    |
| YEAR            | int64, Int64(nullable)      |                                    |
| TINYBLOB        | object                      |                                    |
| BLOB            | object                      |                                    |
| MEDIUMBLOB      | object                      |                                    |
| LONGBLOB        | object                      |                                    |
| JSON            | object                      |                                    |
| ENUM            | object                      |                                    |

## SQLite

SQLite does not need to specify protocol.

### SQLite-Pandas Type Mapping

Since SQLite adopts a [dynamic type system](https://www.sqlite.org/datatype3.html), we infer type as follow:
* If there is a declared type of the column, we derive the type using [column affinity rules](https://www.sqlite.org/datatype3.html#affname), code can be found [here](https://github.com/sfu-db/connector-x/blob/main/connectorx/src/sources/sqlite/typesystem.rs#L47).
* Otherwise we directly adopt the value's type in the first row of the result (in each partition), which results in INTEGER, REAL, TEXT and BLOB.
  * If the first row of the result is NULL in the partition, try next partition. Throw an error if first rows of all partitions are NULL for a column.

| SQLite Type      |      Pandas Type            |  Comment                           |
|:----------------:|:---------------------------:|:----------------------------------:|
| INTEGER          | int64, Int64(nullable)      | declared type that contains substring "int" |
| BOOL             | bool, boolean(nullable)     | declared type is "boolean" or "bool" |
| REAL             | float64                     | declared type that contains substring "real", "floa", "doub" |
| TEXT             | object                      | declared type that contains substring "char", "clob", "text" |
| BLOB             | object                      | declared type that contains substring "blob" |
| DATE             | datetime64[ns]              | declared type is "date"            |
| TIME             | object                      | declared type is "time"            |
| TIMESTAMP        | datetime64[ns]              | declared type is "datetime" or "timestamp" |

## Oracle

Oracle does not need to specify protocol.

### Oracle-Pandas Type Mapping
| Oracle Type               |      Pandas Type            |  Comment                           |
|:-------------------------:|:---------------------------:|:----------------------------------:|
| Number(\*,0)              | int64, Int64(nullable)      |                                    |
| Number(\*,>0)             | float64                     |                                    |
| Float                     | float64                     |                                    |
| BINARY_FLOAT              | float64                     |                                    |
| BINARY_DOUBLE             | float64                     |                                    |
| VARCHAR2                  | object                      |                                    |
| CHAR                      | object                      |                                    |
| NCHAR                     | object                      |                                    |
| NVarchar2                 | object                      |                                    |
| DATE                      | datetime64[ns]              |                                    |
| TIMESTAMP                 | datetime64[ns]              |                                    |
| TIMESTAMP WITH TIME ZONE  | datetime64[ns]              |                                    |

## SQLServer

SQLServer does not need to specify protocol.

By adding `trusted_connection=true` to connection uri parameter, windows authentication will be enabled. Example: `mssql://host:port/db?trusted_connection=true`
By adding `encrypt=true` to connection uri parameter, SQLServer will use SSL encryption. Example: `mssql://host:port/db?encrypt=true&trusted_connection=true`

### SQLServer-Pandas Type Mapping
| SQLServer Type  |      Pandas Type            |  Comment                           |
|:---------------:|:---------------------------:|:----------------------------------:|
| TINYINT         | int64, Int64(nullable)      |                                    |
| SMALLINT        | int64, Int64(nullable)      |                                    |
| INT             | int64, Int64(nullable)      |                                    |
| BIGINT          | int64, Int64(nullable)      |                                    |
| FLOAT           | float64                     |                                    |
| NUMERIC         | float64                     |                                    |
| DECIMAL         | float64                     |                                    |
| BIT             | bool, boolean(nullable)     |                                    |
| VARCHAR         | object                      |                                    |
| CHAR            | object                      |                                    |
| TEXT            | object                      |                                    |
| NVARCHAR        | object                      |                                    |
| NCHAR           | object                      |                                    |
| NTEXT           | object                      |                                    |
| VARBINARY       | object                      |                                    |
| BINARY          | object                      |                                    |
| IMAGE           | object                      |                                    |
| DATETIME        | datetime64[ns]              |                                    |
| DATETIME2       | datetime64[ns]              |                                    |
| SMALLDATETIME   | datetime64[ns]              |                                    |
| DATE            | datetime64[ns]              |                                    |
| DATETIMEOFFSET  | datetime64[ns]              |                                    |
| TIME            | object                      |                                    |
| UNIQUEIDENTIFIER| object                      |                                    |

## Google BigQuery

BigQuery does not need to specify protocol.

### BigQuery-Pandas Type Mapping
| BigQuery Type             |      Pandas Type            |  Comment                           |
|:-------------------------:|:---------------------------:|:----------------------------------:|
| Bool, Boolean             | bool, boolean(nullable)     |                                    |
| Int64, Integer            | int64, Int64(nullable)      |                                    |
| Float64, Float            | float64                     |                                    |
| Numeric                   | float64                     |                                    |
| String                    | object                      |                                    |
| BYTES                     | object                      |                                    |
| Time                      | object                      |                                    |
| DATE                      | datetime64[ns]              |                                    |
| Datetime                  | datetime64[ns]              |                                    |
| TIMESTAMP                 | datetime64[ns]              | UTC                                |
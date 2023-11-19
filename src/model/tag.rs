use diesel::sql_types::Date;
use diesel::prelude::*;



// #[derive(Queryable, Selectable)]
// #[diesel(table_name = crate::schema::posts)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub created_at: Date,
    pub updated_at: Date,
}

// ror: could not compile `diesel_cli` (bin "diesel") due to previous error
// error: failed to compile `diesel_cli v2.1.1`, intermediate artifacts can be found at `/var/folders/69/_n1pn2hx5jz67b85h92dxzy00000gn/T/cargo-installW7IFIw`.
// To reuse those artifacts with a future compilation, set the environment variable `CARGO_TARGET_DIR` to that path.

// export CARGO_TARGET_DIR=/var/folders/69/_n1pn2hx5jz67b85h92dxzy00000gn/T/cargo-installW7IFIw

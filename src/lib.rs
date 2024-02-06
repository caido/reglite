mod cache;
mod operator;
mod wrapper;

use sqlite_loadable::prelude::*;
use sqlite_loadable::{define_scalar_function, errors::Result, FunctionFlags};

#[sqlite_entrypoint]
pub fn sqlite3_reglite_init(db: *mut sqlite3) -> Result<()> {
    let flags = FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC;

    define_scalar_function(db, "regexp", 2, operator::regexp, flags)?;

    Ok(())
}

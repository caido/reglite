use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};
use std::os::raw::c_void;

use crate::wrapper::Regex;

pub enum RegexSource {
    Query(i32),
    Auxdata,
}

pub fn get_regex(
    ctx: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
    col: i32,
) -> Result<(*mut Regex, RegexSource)> {
    let auxdata = api::auxdata_get(ctx, col);
    if !auxdata.is_null() {
        Ok((auxdata.cast::<Regex>(), RegexSource::Auxdata))
    } else {
        let value = values
            .get(col as usize)
            .ok_or_else(|| Error::new_message(format!("expected argument {col} as pattern")))?;

        let pattern = api::value_text_notnull(value)?;
        let boxed = Box::new(
            Regex::new(pattern).map_err(|_| Error::new_message("pattern not valid regex"))?,
        );
        Ok((Box::into_raw(boxed), RegexSource::Query(col)))
    }
}

unsafe extern "C" fn cleanup_regex(arg: *mut c_void) {
    drop(Box::from_raw(arg.cast::<Regex>()))
}

pub fn set_regex(ctx: *mut sqlite3_context, regex: *mut Regex, source: RegexSource) {
    match source {
        RegexSource::Auxdata => {}
        RegexSource::Query(col) => {
            api::auxdata_set(ctx, col, regex.cast::<c_void>(), Some(cleanup_regex))
        }
    }
}

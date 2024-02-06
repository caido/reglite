use crate::cache;
use sqlite_loadable::api::ValueType;
use sqlite_loadable::ext::{sqlite3_context, sqlite3_value};
use sqlite_loadable::{api, Error, Result};

/// Use as `regexp(pattern, col)` or `col REGEXP pattern`
pub fn regexp(ctx: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    // Get content or return early if null
    let content_value = values.get(1).ok_or("expected argument 1 as content")?;
    let content = match api::value_type(content_value) {
        ValueType::Null => {
            api::result_bool(ctx, false);
            return Ok(());
        }
        ValueType::Blob | ValueType::Text => api::value_blob(content_value),
        _ => {
            return Err(Error::new_message(
                "expected argument 1 as content of type blob or text",
            ));
        }
    };

    // Get regex
    let (regex, source) = cache::get_regex(ctx, values, 0)?;
    let regex = unsafe { &mut *regex };

    // Match and save
    api::result_bool(ctx, regex.is_match(content));
    cache::set_regex(ctx, regex, source);
    Ok(())
}

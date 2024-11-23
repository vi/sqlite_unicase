use std::cmp::Ordering;

use rusqlite::{ffi, Connection};
use unicase::UniCase;

use std::os::raw::{c_char, c_int};

use rusqlite::Result;

// Source: https://github.com/ankitects/anki/blob/3c20b49ccac4da63a85bf65b89af845d1f4851fb/rslib/src/storage/sqlite.rs#L33
fn unicase_compare(s1: &str, s2: &str) -> Ordering {
    UniCase::new(s1).cmp(&UniCase::new(s2))
}

/// Entry point for SQLite to load the extension.
/// See <https://sqlite.org/c3ref/load_extension.html> on this function's name and usage.
/// # Safety
/// This function is called by SQLite and must be safe to call.
#[expect(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub unsafe extern "C" fn sqlite3_extension_init(
    db: *mut ffi::sqlite3,
    pz_err_msg: *mut *mut c_char,
    p_api: *mut ffi::sqlite3_api_routines,
) -> c_int {
    Connection::extension_init2(db, pz_err_msg, p_api, extension_init)
}


fn extension_init(db: Connection) -> Result<bool> {
    db.create_collation("unicase", unicase_compare)?;
    Ok(false)
}

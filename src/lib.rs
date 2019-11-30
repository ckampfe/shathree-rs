#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types, ptr_wrapping_offset_from)]

extern "C" {
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_str;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_stmt;
    pub type sqlite3_blob;
    pub type sqlite3_backup;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type sqlite_int64 = libc::c_longlong;
pub type sqlite_uint64 = libc::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type sqlite3_callback = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_file {
    pub pMethods: *const sqlite3_io_methods,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_io_methods {
    pub iVersion: libc::c_int,
    pub xClose: Option<unsafe extern "C" fn(_: *mut sqlite3_file) -> libc::c_int>,
    pub xRead: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_file,
            _: *mut libc::c_void,
            _: libc::c_int,
            _: sqlite3_int64,
        ) -> libc::c_int,
    >,
    pub xWrite: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_file,
            _: *const libc::c_void,
            _: libc::c_int,
            _: sqlite3_int64,
        ) -> libc::c_int,
    >,
    pub xTruncate:
        Option<unsafe extern "C" fn(_: *mut sqlite3_file, _: sqlite3_int64) -> libc::c_int>,
    pub xSync: Option<unsafe extern "C" fn(_: *mut sqlite3_file, _: libc::c_int) -> libc::c_int>,
    pub xFileSize:
        Option<unsafe extern "C" fn(_: *mut sqlite3_file, _: *mut sqlite3_int64) -> libc::c_int>,
    pub xLock: Option<unsafe extern "C" fn(_: *mut sqlite3_file, _: libc::c_int) -> libc::c_int>,
    pub xUnlock: Option<unsafe extern "C" fn(_: *mut sqlite3_file, _: libc::c_int) -> libc::c_int>,
    pub xCheckReservedLock:
        Option<unsafe extern "C" fn(_: *mut sqlite3_file, _: *mut libc::c_int) -> libc::c_int>,
    pub xFileControl: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_file,
            _: libc::c_int,
            _: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub xSectorSize: Option<unsafe extern "C" fn(_: *mut sqlite3_file) -> libc::c_int>,
    pub xDeviceCharacteristics: Option<unsafe extern "C" fn(_: *mut sqlite3_file) -> libc::c_int>,
    pub xShmMap: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_file,
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
            _: *mut *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub xShmLock: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_file,
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub xShmBarrier: Option<unsafe extern "C" fn(_: *mut sqlite3_file) -> ()>,
    pub xShmUnmap:
        Option<unsafe extern "C" fn(_: *mut sqlite3_file, _: libc::c_int) -> libc::c_int>,
    pub xFetch: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_file,
            _: sqlite3_int64,
            _: libc::c_int,
            _: *mut *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub xUnfetch: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_file,
            _: sqlite3_int64,
            _: *mut libc::c_void,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_api_routines {
    pub aggregate_context:
        Option<unsafe extern "C" fn(_: *mut sqlite3_context, _: libc::c_int) -> *mut libc::c_void>,
    pub aggregate_count: Option<unsafe extern "C" fn(_: *mut sqlite3_context) -> libc::c_int>,
    pub bind_blob: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_stmt,
            _: libc::c_int,
            _: *const libc::c_void,
            _: libc::c_int,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> libc::c_int,
    >,
    pub bind_double: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_stmt,
            _: libc::c_int,
            _: libc::c_double,
        ) -> libc::c_int,
    >,
    pub bind_int: Option<
        unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int, _: libc::c_int) -> libc::c_int,
    >,
    pub bind_int64: Option<
        unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int, _: sqlite_int64) -> libc::c_int,
    >,
    pub bind_null:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> libc::c_int>,
    pub bind_parameter_count: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> libc::c_int>,
    pub bind_parameter_index:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: *const libc::c_char) -> libc::c_int>,
    pub bind_parameter_name:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_char>,
    pub bind_text: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_stmt,
            _: libc::c_int,
            _: *const libc::c_char,
            _: libc::c_int,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> libc::c_int,
    >,
    pub bind_text16: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_stmt,
            _: libc::c_int,
            _: *const libc::c_void,
            _: libc::c_int,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> libc::c_int,
    >,
    pub bind_value: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_stmt,
            _: libc::c_int,
            _: *const sqlite3_value,
        ) -> libc::c_int,
    >,
    pub busy_handler: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> libc::c_int>,
            _: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub busy_timeout: Option<unsafe extern "C" fn(_: *mut sqlite3, _: libc::c_int) -> libc::c_int>,
    pub changes: Option<unsafe extern "C" fn(_: *mut sqlite3) -> libc::c_int>,
    pub close: Option<unsafe extern "C" fn(_: *mut sqlite3) -> libc::c_int>,
    pub collation_needed: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *mut libc::c_void,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut sqlite3,
                    _: libc::c_int,
                    _: *const libc::c_char,
                ) -> (),
            >,
        ) -> libc::c_int,
    >,
    pub collation_needed16: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *mut libc::c_void,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut sqlite3,
                    _: libc::c_int,
                    _: *const libc::c_void,
                ) -> (),
            >,
        ) -> libc::c_int,
    >,
    pub column_blob:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_void>,
    pub column_bytes:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> libc::c_int>,
    pub column_bytes16:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> libc::c_int>,
    pub column_count: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> libc::c_int>,
    pub column_database_name:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_char>,
    pub column_database_name16:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_void>,
    pub column_decltype:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_char>,
    pub column_decltype16:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_void>,
    pub column_double:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> libc::c_double>,
    pub column_int:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> libc::c_int>,
    pub column_int64:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> sqlite_int64>,
    pub column_name:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_char>,
    pub column_name16:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_void>,
    pub column_origin_name:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_char>,
    pub column_origin_name16:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_void>,
    pub column_table_name:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_char>,
    pub column_table_name16:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_void>,
    pub column_text:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_uchar>,
    pub column_text16:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *const libc::c_void>,
    pub column_type:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> libc::c_int>,
    pub column_value:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int) -> *mut sqlite3_value>,
    pub commit_hook: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
            _: *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub complete: Option<unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int>,
    pub complete16: Option<unsafe extern "C" fn(_: *const libc::c_void) -> libc::c_int>,
    pub create_collation: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: libc::c_int,
            _: *mut libc::c_void,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: libc::c_int,
                    _: *const libc::c_void,
                    _: libc::c_int,
                    _: *const libc::c_void,
                ) -> libc::c_int,
            >,
        ) -> libc::c_int,
    >,
    pub create_collation16: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_void,
            _: libc::c_int,
            _: *mut libc::c_void,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: libc::c_int,
                    _: *const libc::c_void,
                    _: libc::c_int,
                    _: *const libc::c_void,
                ) -> libc::c_int,
            >,
        ) -> libc::c_int,
    >,
    pub create_function: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: libc::c_int,
            _: libc::c_int,
            _: *mut libc::c_void,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut sqlite3_context,
                    _: libc::c_int,
                    _: *mut *mut sqlite3_value,
                ) -> (),
            >,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut sqlite3_context,
                    _: libc::c_int,
                    _: *mut *mut sqlite3_value,
                ) -> (),
            >,
            _: Option<unsafe extern "C" fn(_: *mut sqlite3_context) -> ()>,
        ) -> libc::c_int,
    >,
    pub create_function16: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_void,
            _: libc::c_int,
            _: libc::c_int,
            _: *mut libc::c_void,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut sqlite3_context,
                    _: libc::c_int,
                    _: *mut *mut sqlite3_value,
                ) -> (),
            >,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut sqlite3_context,
                    _: libc::c_int,
                    _: *mut *mut sqlite3_value,
                ) -> (),
            >,
            _: Option<unsafe extern "C" fn(_: *mut sqlite3_context) -> ()>,
        ) -> libc::c_int,
    >,
    pub create_module: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: *const sqlite3_module,
            _: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub data_count: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> libc::c_int>,
    pub db_handle: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> *mut sqlite3>,
    pub declare_vtab:
        Option<unsafe extern "C" fn(_: *mut sqlite3, _: *const libc::c_char) -> libc::c_int>,
    pub enable_shared_cache: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
    pub errcode: Option<unsafe extern "C" fn(_: *mut sqlite3) -> libc::c_int>,
    pub errmsg: Option<unsafe extern "C" fn(_: *mut sqlite3) -> *const libc::c_char>,
    pub errmsg16: Option<unsafe extern "C" fn(_: *mut sqlite3) -> *const libc::c_void>,
    pub exec: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: sqlite3_callback,
            _: *mut libc::c_void,
            _: *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub expired: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> libc::c_int>,
    pub finalize: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> libc::c_int>,
    pub free: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub free_table: Option<unsafe extern "C" fn(_: *mut *mut libc::c_char) -> ()>,
    pub get_autocommit: Option<unsafe extern "C" fn(_: *mut sqlite3) -> libc::c_int>,
    pub get_auxdata:
        Option<unsafe extern "C" fn(_: *mut sqlite3_context, _: libc::c_int) -> *mut libc::c_void>,
    pub get_table: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: *mut *mut *mut libc::c_char,
            _: *mut libc::c_int,
            _: *mut libc::c_int,
            _: *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub global_recover: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub interruptx: Option<unsafe extern "C" fn(_: *mut sqlite3) -> ()>,
    pub last_insert_rowid: Option<unsafe extern "C" fn(_: *mut sqlite3) -> sqlite_int64>,
    pub libversion: Option<unsafe extern "C" fn() -> *const libc::c_char>,
    pub libversion_number: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub malloc: Option<unsafe extern "C" fn(_: libc::c_int) -> *mut libc::c_void>,
    pub mprintf: Option<unsafe extern "C" fn(_: *const libc::c_char, _: ...) -> *mut libc::c_char>,
    pub open:
        Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut *mut sqlite3) -> libc::c_int>,
    pub open16:
        Option<unsafe extern "C" fn(_: *const libc::c_void, _: *mut *mut sqlite3) -> libc::c_int>,
    pub prepare: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: libc::c_int,
            _: *mut *mut sqlite3_stmt,
            _: *mut *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub prepare16: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_void,
            _: libc::c_int,
            _: *mut *mut sqlite3_stmt,
            _: *mut *const libc::c_void,
        ) -> libc::c_int,
    >,
    pub profile: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *const libc::c_char,
                    _: sqlite_uint64,
                ) -> (),
            >,
            _: *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub progress_handler: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: libc::c_int,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub realloc:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> *mut libc::c_void>,
    pub reset: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> libc::c_int>,
    pub result_blob: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_context,
            _: *const libc::c_void,
            _: libc::c_int,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> (),
    >,
    pub result_double:
        Option<unsafe extern "C" fn(_: *mut sqlite3_context, _: libc::c_double) -> ()>,
    pub result_error: Option<
        unsafe extern "C" fn(_: *mut sqlite3_context, _: *const libc::c_char, _: libc::c_int) -> (),
    >,
    pub result_error16: Option<
        unsafe extern "C" fn(_: *mut sqlite3_context, _: *const libc::c_void, _: libc::c_int) -> (),
    >,
    pub result_int: Option<unsafe extern "C" fn(_: *mut sqlite3_context, _: libc::c_int) -> ()>,
    pub result_int64: Option<unsafe extern "C" fn(_: *mut sqlite3_context, _: sqlite_int64) -> ()>,
    pub result_null: Option<unsafe extern "C" fn(_: *mut sqlite3_context) -> ()>,
    pub result_text: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_context,
            _: *const libc::c_char,
            _: libc::c_int,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> (),
    >,
    pub result_text16: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_context,
            _: *const libc::c_void,
            _: libc::c_int,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> (),
    >,
    pub result_text16be: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_context,
            _: *const libc::c_void,
            _: libc::c_int,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> (),
    >,
    pub result_text16le: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_context,
            _: *const libc::c_void,
            _: libc::c_int,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> (),
    >,
    pub result_value:
        Option<unsafe extern "C" fn(_: *mut sqlite3_context, _: *mut sqlite3_value) -> ()>,
    pub rollback_hook: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
            _: *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub set_authorizer: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: libc::c_int,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
            >,
            _: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub set_auxdata: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_context,
            _: libc::c_int,
            _: *mut libc::c_void,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> (),
    >,
    pub xsnprintf: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: *mut libc::c_char,
            _: *const libc::c_char,
            _: ...
        ) -> *mut libc::c_char,
    >,
    pub step: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> libc::c_int>,
    pub table_column_metadata: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: *mut *const libc::c_char,
            _: *mut *const libc::c_char,
            _: *mut libc::c_int,
            _: *mut libc::c_int,
            _: *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub thread_cleanup: Option<unsafe extern "C" fn() -> ()>,
    pub total_changes: Option<unsafe extern "C" fn(_: *mut sqlite3) -> libc::c_int>,
    pub trace: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char) -> ()>,
            _: *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub transfer_bindings:
        Option<unsafe extern "C" fn(_: *mut sqlite3_stmt, _: *mut sqlite3_stmt) -> libc::c_int>,
    pub update_hook: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: libc::c_int,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: sqlite_int64,
                ) -> (),
            >,
            _: *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub user_data: Option<unsafe extern "C" fn(_: *mut sqlite3_context) -> *mut libc::c_void>,
    pub value_blob: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> *const libc::c_void>,
    pub value_bytes: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> libc::c_int>,
    pub value_bytes16: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> libc::c_int>,
    pub value_double: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> libc::c_double>,
    pub value_int: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> libc::c_int>,
    pub value_int64: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> sqlite_int64>,
    pub value_numeric_type: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> libc::c_int>,
    pub value_text: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> *const libc::c_uchar>,
    pub value_text16: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> *const libc::c_void>,
    pub value_text16be: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> *const libc::c_void>,
    pub value_text16le: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> *const libc::c_void>,
    pub value_type: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> libc::c_int>,
    pub vmprintf: Option<
        unsafe extern "C" fn(_: *const libc::c_char, _: ::std::ffi::VaList) -> *mut libc::c_char,
    >,
    pub overload_function: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub prepare_v2: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: libc::c_int,
            _: *mut *mut sqlite3_stmt,
            _: *mut *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub prepare16_v2: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_void,
            _: libc::c_int,
            _: *mut *mut sqlite3_stmt,
            _: *mut *const libc::c_void,
        ) -> libc::c_int,
    >,
    pub clear_bindings: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> libc::c_int>,
    pub create_module_v2: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: *const sqlite3_module,
            _: *mut libc::c_void,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> libc::c_int,
    >,
    pub bind_zeroblob: Option<
        unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int, _: libc::c_int) -> libc::c_int,
    >,
    pub blob_bytes: Option<unsafe extern "C" fn(_: *mut sqlite3_blob) -> libc::c_int>,
    pub blob_close: Option<unsafe extern "C" fn(_: *mut sqlite3_blob) -> libc::c_int>,
    pub blob_open: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: sqlite3_int64,
            _: libc::c_int,
            _: *mut *mut sqlite3_blob,
        ) -> libc::c_int,
    >,
    pub blob_read: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_blob,
            _: *mut libc::c_void,
            _: libc::c_int,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub blob_write: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_blob,
            _: *const libc::c_void,
            _: libc::c_int,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub create_collation_v2: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: libc::c_int,
            _: *mut libc::c_void,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: libc::c_int,
                    _: *const libc::c_void,
                    _: libc::c_int,
                    _: *const libc::c_void,
                ) -> libc::c_int,
            >,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> libc::c_int,
    >,
    pub file_control: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: libc::c_int,
            _: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub memory_highwater: Option<unsafe extern "C" fn(_: libc::c_int) -> sqlite3_int64>,
    pub memory_used: Option<unsafe extern "C" fn() -> sqlite3_int64>,
    pub mutex_alloc: Option<unsafe extern "C" fn(_: libc::c_int) -> *mut sqlite3_mutex>,
    pub mutex_enter: Option<unsafe extern "C" fn(_: *mut sqlite3_mutex) -> ()>,
    pub mutex_free: Option<unsafe extern "C" fn(_: *mut sqlite3_mutex) -> ()>,
    pub mutex_leave: Option<unsafe extern "C" fn(_: *mut sqlite3_mutex) -> ()>,
    pub mutex_try: Option<unsafe extern "C" fn(_: *mut sqlite3_mutex) -> libc::c_int>,
    pub open_v2: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut *mut sqlite3,
            _: libc::c_int,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub release_memory: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
    pub result_error_nomem: Option<unsafe extern "C" fn(_: *mut sqlite3_context) -> ()>,
    pub result_error_toobig: Option<unsafe extern "C" fn(_: *mut sqlite3_context) -> ()>,
    pub sleep: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
    pub soft_heap_limit: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub vfs_find: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut sqlite3_vfs>,
    pub vfs_register:
        Option<unsafe extern "C" fn(_: *mut sqlite3_vfs, _: libc::c_int) -> libc::c_int>,
    pub vfs_unregister: Option<unsafe extern "C" fn(_: *mut sqlite3_vfs) -> libc::c_int>,
    pub xthreadsafe: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub result_zeroblob:
        Option<unsafe extern "C" fn(_: *mut sqlite3_context, _: libc::c_int) -> ()>,
    pub result_error_code:
        Option<unsafe extern "C" fn(_: *mut sqlite3_context, _: libc::c_int) -> ()>,
    pub test_control: Option<unsafe extern "C" fn(_: libc::c_int, _: ...) -> libc::c_int>,
    pub randomness: Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_void) -> ()>,
    pub context_db_handle: Option<unsafe extern "C" fn(_: *mut sqlite3_context) -> *mut sqlite3>,
    pub extended_result_codes:
        Option<unsafe extern "C" fn(_: *mut sqlite3, _: libc::c_int) -> libc::c_int>,
    pub limit: Option<
        unsafe extern "C" fn(_: *mut sqlite3, _: libc::c_int, _: libc::c_int) -> libc::c_int,
    >,
    pub next_stmt:
        Option<unsafe extern "C" fn(_: *mut sqlite3, _: *mut sqlite3_stmt) -> *mut sqlite3_stmt>,
    pub sql: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> *const libc::c_char>,
    pub status: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: *mut libc::c_int,
            _: *mut libc::c_int,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub backup_finish: Option<unsafe extern "C" fn(_: *mut sqlite3_backup) -> libc::c_int>,
    pub backup_init: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: *mut sqlite3,
            _: *const libc::c_char,
        ) -> *mut sqlite3_backup,
    >,
    pub backup_pagecount: Option<unsafe extern "C" fn(_: *mut sqlite3_backup) -> libc::c_int>,
    pub backup_remaining: Option<unsafe extern "C" fn(_: *mut sqlite3_backup) -> libc::c_int>,
    pub backup_step:
        Option<unsafe extern "C" fn(_: *mut sqlite3_backup, _: libc::c_int) -> libc::c_int>,
    pub compileoption_get: Option<unsafe extern "C" fn(_: libc::c_int) -> *const libc::c_char>,
    pub compileoption_used: Option<unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int>,
    pub create_function_v2: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: libc::c_int,
            _: libc::c_int,
            _: *mut libc::c_void,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut sqlite3_context,
                    _: libc::c_int,
                    _: *mut *mut sqlite3_value,
                ) -> (),
            >,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut sqlite3_context,
                    _: libc::c_int,
                    _: *mut *mut sqlite3_value,
                ) -> (),
            >,
            _: Option<unsafe extern "C" fn(_: *mut sqlite3_context) -> ()>,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> libc::c_int,
    >,
    pub db_config:
        Option<unsafe extern "C" fn(_: *mut sqlite3, _: libc::c_int, _: ...) -> libc::c_int>,
    pub db_mutex: Option<unsafe extern "C" fn(_: *mut sqlite3) -> *mut sqlite3_mutex>,
    pub db_status: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: libc::c_int,
            _: *mut libc::c_int,
            _: *mut libc::c_int,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub extended_errcode: Option<unsafe extern "C" fn(_: *mut sqlite3) -> libc::c_int>,
    pub log: Option<unsafe extern "C" fn(_: libc::c_int, _: *const libc::c_char, _: ...) -> ()>,
    pub soft_heap_limit64: Option<unsafe extern "C" fn(_: sqlite3_int64) -> sqlite3_int64>,
    pub sourceid: Option<unsafe extern "C" fn() -> *const libc::c_char>,
    pub stmt_status: Option<
        unsafe extern "C" fn(_: *mut sqlite3_stmt, _: libc::c_int, _: libc::c_int) -> libc::c_int,
    >,
    pub strnicmp: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub unlock_notify: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: Option<unsafe extern "C" fn(_: *mut *mut libc::c_void, _: libc::c_int) -> ()>,
            _: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub wal_autocheckpoint:
        Option<unsafe extern "C" fn(_: *mut sqlite3, _: libc::c_int) -> libc::c_int>,
    pub wal_checkpoint:
        Option<unsafe extern "C" fn(_: *mut sqlite3, _: *const libc::c_char) -> libc::c_int>,
    pub wal_hook: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: Option<
                unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut sqlite3,
                    _: *const libc::c_char,
                    _: libc::c_int,
                ) -> libc::c_int,
            >,
            _: *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub blob_reopen:
        Option<unsafe extern "C" fn(_: *mut sqlite3_blob, _: sqlite3_int64) -> libc::c_int>,
    pub vtab_config:
        Option<unsafe extern "C" fn(_: *mut sqlite3, _: libc::c_int, _: ...) -> libc::c_int>,
    pub vtab_on_conflict: Option<unsafe extern "C" fn(_: *mut sqlite3) -> libc::c_int>,
    pub close_v2: Option<unsafe extern "C" fn(_: *mut sqlite3) -> libc::c_int>,
    pub db_filename: Option<
        unsafe extern "C" fn(_: *mut sqlite3, _: *const libc::c_char) -> *const libc::c_char,
    >,
    pub db_readonly:
        Option<unsafe extern "C" fn(_: *mut sqlite3, _: *const libc::c_char) -> libc::c_int>,
    pub db_release_memory: Option<unsafe extern "C" fn(_: *mut sqlite3) -> libc::c_int>,
    pub errstr: Option<unsafe extern "C" fn(_: libc::c_int) -> *const libc::c_char>,
    pub stmt_busy: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> libc::c_int>,
    pub stmt_readonly: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> libc::c_int>,
    pub stricmp:
        Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int>,
    pub uri_boolean: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub uri_int64: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: sqlite3_int64,
        ) -> sqlite3_int64,
    >,
    pub uri_parameter: Option<
        unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> *const libc::c_char,
    >,
    pub xvsnprintf: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: *mut libc::c_char,
            _: *const libc::c_char,
            _: ::std::ffi::VaList,
        ) -> *mut libc::c_char,
    >,
    pub wal_checkpoint_v2: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: libc::c_int,
            _: *mut libc::c_int,
            _: *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub auto_extension:
        Option<unsafe extern "C" fn(_: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int>,
    pub bind_blob64: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_stmt,
            _: libc::c_int,
            _: *const libc::c_void,
            _: sqlite3_uint64,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> libc::c_int,
    >,
    pub bind_text64: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_stmt,
            _: libc::c_int,
            _: *const libc::c_char,
            _: sqlite3_uint64,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
            _: libc::c_uchar,
        ) -> libc::c_int,
    >,
    pub cancel_auto_extension:
        Option<unsafe extern "C" fn(_: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int>,
    pub load_extension: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub malloc64: Option<unsafe extern "C" fn(_: sqlite3_uint64) -> *mut libc::c_void>,
    pub msize: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> sqlite3_uint64>,
    pub realloc64:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: sqlite3_uint64) -> *mut libc::c_void>,
    pub reset_auto_extension: Option<unsafe extern "C" fn() -> ()>,
    pub result_blob64: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_context,
            _: *const libc::c_void,
            _: sqlite3_uint64,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> (),
    >,
    pub result_text64: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_context,
            _: *const libc::c_char,
            _: sqlite3_uint64,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
            _: libc::c_uchar,
        ) -> (),
    >,
    pub strglob:
        Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int>,
    pub value_dup: Option<unsafe extern "C" fn(_: *const sqlite3_value) -> *mut sqlite3_value>,
    pub value_free: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> ()>,
    pub result_zeroblob64:
        Option<unsafe extern "C" fn(_: *mut sqlite3_context, _: sqlite3_uint64) -> libc::c_int>,
    pub bind_zeroblob64: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_stmt,
            _: libc::c_int,
            _: sqlite3_uint64,
        ) -> libc::c_int,
    >,
    pub value_subtype: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> libc::c_uint>,
    pub result_subtype:
        Option<unsafe extern "C" fn(_: *mut sqlite3_context, _: libc::c_uint) -> ()>,
    pub status64: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: *mut sqlite3_int64,
            _: *mut sqlite3_int64,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub strlike: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: libc::c_uint,
        ) -> libc::c_int,
    >,
    pub db_cacheflush: Option<unsafe extern "C" fn(_: *mut sqlite3) -> libc::c_int>,
    pub system_errno: Option<unsafe extern "C" fn(_: *mut sqlite3) -> libc::c_int>,
    pub trace_v2: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: libc::c_uint,
            _: Option<
                unsafe extern "C" fn(
                    _: libc::c_uint,
                    _: *mut libc::c_void,
                    _: *mut libc::c_void,
                    _: *mut libc::c_void,
                ) -> libc::c_int,
            >,
            _: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub expanded_sql: Option<unsafe extern "C" fn(_: *mut sqlite3_stmt) -> *mut libc::c_char>,
    pub set_last_insert_rowid:
        Option<unsafe extern "C" fn(_: *mut sqlite3, _: sqlite3_int64) -> ()>,
    pub prepare_v3: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_char,
            _: libc::c_int,
            _: libc::c_uint,
            _: *mut *mut sqlite3_stmt,
            _: *mut *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub prepare16_v3: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *const libc::c_void,
            _: libc::c_int,
            _: libc::c_uint,
            _: *mut *mut sqlite3_stmt,
            _: *mut *const libc::c_void,
        ) -> libc::c_int,
    >,
    pub bind_pointer: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_stmt,
            _: libc::c_int,
            _: *mut libc::c_void,
            _: *const libc::c_char,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> libc::c_int,
    >,
    pub result_pointer: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_context,
            _: *mut libc::c_void,
            _: *const libc::c_char,
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        ) -> (),
    >,
    pub value_pointer: Option<
        unsafe extern "C" fn(_: *mut sqlite3_value, _: *const libc::c_char) -> *mut libc::c_void,
    >,
    pub vtab_nochange: Option<unsafe extern "C" fn(_: *mut sqlite3_context) -> libc::c_int>,
    pub value_nochange: Option<unsafe extern "C" fn(_: *mut sqlite3_value) -> libc::c_int>,
    pub vtab_collation: Option<
        unsafe extern "C" fn(_: *mut sqlite3_index_info, _: libc::c_int) -> *const libc::c_char,
    >,
    pub keyword_count: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub keyword_name: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: *mut *const libc::c_char,
            _: *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub keyword_check:
        Option<unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int) -> libc::c_int>,
    pub str_new: Option<unsafe extern "C" fn(_: *mut sqlite3) -> *mut sqlite3_str>,
    pub str_finish: Option<unsafe extern "C" fn(_: *mut sqlite3_str) -> *mut libc::c_char>,
    pub str_appendf:
        Option<unsafe extern "C" fn(_: *mut sqlite3_str, _: *const libc::c_char, _: ...) -> ()>,
    pub str_vappendf: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_str,
            _: *const libc::c_char,
            _: ::std::ffi::VaList,
        ) -> (),
    >,
    pub str_append: Option<
        unsafe extern "C" fn(_: *mut sqlite3_str, _: *const libc::c_char, _: libc::c_int) -> (),
    >,
    pub str_appendall:
        Option<unsafe extern "C" fn(_: *mut sqlite3_str, _: *const libc::c_char) -> ()>,
    pub str_appendchar:
        Option<unsafe extern "C" fn(_: *mut sqlite3_str, _: libc::c_int, _: libc::c_char) -> ()>,
    pub str_reset: Option<unsafe extern "C" fn(_: *mut sqlite3_str) -> ()>,
    pub str_errcode: Option<unsafe extern "C" fn(_: *mut sqlite3_str) -> libc::c_int>,
    pub str_length: Option<unsafe extern "C" fn(_: *mut sqlite3_str) -> libc::c_int>,
    pub str_value: Option<unsafe extern "C" fn(_: *mut sqlite3_str) -> *mut libc::c_char>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_info {
    pub nConstraint: libc::c_int,
    pub aConstraint: *mut sqlite3_index_constraint,
    pub nOrderBy: libc::c_int,
    pub aOrderBy: *mut sqlite3_index_orderby,
    pub aConstraintUsage: *mut sqlite3_index_constraint_usage,
    pub idxNum: libc::c_int,
    pub idxStr: *mut libc::c_char,
    pub needToFreeIdxStr: libc::c_int,
    pub orderByConsumed: libc::c_int,
    pub estimatedCost: libc::c_double,
    pub estimatedRows: sqlite3_int64,
    pub idxFlags: libc::c_int,
    pub colUsed: sqlite3_uint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint_usage {
    pub argvIndex: libc::c_int,
    pub omit: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_orderby {
    pub iColumn: libc::c_int,
    pub desc: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint {
    pub iColumn: libc::c_int,
    pub op: libc::c_uchar,
    pub usable: libc::c_uchar,
    pub iTermOffset: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vfs {
    pub iVersion: libc::c_int,
    pub szOsFile: libc::c_int,
    pub mxPathname: libc::c_int,
    pub pNext: *mut sqlite3_vfs,
    pub zName: *const libc::c_char,
    pub pAppData: *mut libc::c_void,
    pub xOpen: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_vfs,
            _: *const libc::c_char,
            _: *mut sqlite3_file,
            _: libc::c_int,
            _: *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub xDelete: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_vfs,
            _: *const libc::c_char,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub xAccess: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_vfs,
            _: *const libc::c_char,
            _: libc::c_int,
            _: *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub xFullPathname: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_vfs,
            _: *const libc::c_char,
            _: libc::c_int,
            _: *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub xDlOpen: Option<
        unsafe extern "C" fn(_: *mut sqlite3_vfs, _: *const libc::c_char) -> *mut libc::c_void,
    >,
    pub xDlError: Option<
        unsafe extern "C" fn(_: *mut sqlite3_vfs, _: libc::c_int, _: *mut libc::c_char) -> (),
    >,
    pub xDlSym: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_vfs,
            _: *mut libc::c_void,
            _: *const libc::c_char,
        ) -> Option<unsafe extern "C" fn() -> ()>,
    >,
    pub xDlClose: Option<unsafe extern "C" fn(_: *mut sqlite3_vfs, _: *mut libc::c_void) -> ()>,
    pub xRandomness: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_vfs,
            _: libc::c_int,
            _: *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub xSleep: Option<unsafe extern "C" fn(_: *mut sqlite3_vfs, _: libc::c_int) -> libc::c_int>,
    pub xCurrentTime:
        Option<unsafe extern "C" fn(_: *mut sqlite3_vfs, _: *mut libc::c_double) -> libc::c_int>,
    pub xGetLastError: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_vfs,
            _: libc::c_int,
            _: *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub xCurrentTimeInt64:
        Option<unsafe extern "C" fn(_: *mut sqlite3_vfs, _: *mut sqlite3_int64) -> libc::c_int>,
    pub xSetSystemCall: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_vfs,
            _: *const libc::c_char,
            _: sqlite3_syscall_ptr,
        ) -> libc::c_int,
    >,
    pub xGetSystemCall: Option<
        unsafe extern "C" fn(_: *mut sqlite3_vfs, _: *const libc::c_char) -> sqlite3_syscall_ptr,
    >,
    pub xNextSystemCall: Option<
        unsafe extern "C" fn(_: *mut sqlite3_vfs, _: *const libc::c_char) -> *const libc::c_char,
    >,
}
pub type sqlite3_syscall_ptr = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_module {
    pub iVersion: libc::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *mut libc::c_void,
            _: libc::c_int,
            _: *const *const libc::c_char,
            _: *mut *mut sqlite3_vtab,
            _: *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub xConnect: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3,
            _: *mut libc::c_void,
            _: libc::c_int,
            _: *const *const libc::c_char,
            _: *mut *mut sqlite3_vtab,
            _: *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub xBestIndex: Option<
        unsafe extern "C" fn(_: *mut sqlite3_vtab, _: *mut sqlite3_index_info) -> libc::c_int,
    >,
    pub xDisconnect: Option<unsafe extern "C" fn(_: *mut sqlite3_vtab) -> libc::c_int>,
    pub xDestroy: Option<unsafe extern "C" fn(_: *mut sqlite3_vtab) -> libc::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(_: *mut sqlite3_vtab, _: *mut *mut sqlite3_vtab_cursor) -> libc::c_int,
    >,
    pub xClose: Option<unsafe extern "C" fn(_: *mut sqlite3_vtab_cursor) -> libc::c_int>,
    pub xFilter: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_vtab_cursor,
            _: libc::c_int,
            _: *const libc::c_char,
            _: libc::c_int,
            _: *mut *mut sqlite3_value,
        ) -> libc::c_int,
    >,
    pub xNext: Option<unsafe extern "C" fn(_: *mut sqlite3_vtab_cursor) -> libc::c_int>,
    pub xEof: Option<unsafe extern "C" fn(_: *mut sqlite3_vtab_cursor) -> libc::c_int>,
    pub xColumn: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_vtab_cursor,
            _: *mut sqlite3_context,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub xRowid: Option<
        unsafe extern "C" fn(_: *mut sqlite3_vtab_cursor, _: *mut sqlite3_int64) -> libc::c_int,
    >,
    pub xUpdate: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_vtab,
            _: libc::c_int,
            _: *mut *mut sqlite3_value,
            _: *mut sqlite3_int64,
        ) -> libc::c_int,
    >,
    pub xBegin: Option<unsafe extern "C" fn(_: *mut sqlite3_vtab) -> libc::c_int>,
    pub xSync: Option<unsafe extern "C" fn(_: *mut sqlite3_vtab) -> libc::c_int>,
    pub xCommit: Option<unsafe extern "C" fn(_: *mut sqlite3_vtab) -> libc::c_int>,
    pub xRollback: Option<unsafe extern "C" fn(_: *mut sqlite3_vtab) -> libc::c_int>,
    pub xFindFunction: Option<
        unsafe extern "C" fn(
            _: *mut sqlite3_vtab,
            _: libc::c_int,
            _: *const libc::c_char,
            _: *mut Option<
                unsafe extern "C" fn(
                    _: *mut sqlite3_context,
                    _: libc::c_int,
                    _: *mut *mut sqlite3_value,
                ) -> (),
            >,
            _: *mut *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub xRename:
        Option<unsafe extern "C" fn(_: *mut sqlite3_vtab, _: *const libc::c_char) -> libc::c_int>,
    pub xSavepoint:
        Option<unsafe extern "C" fn(_: *mut sqlite3_vtab, _: libc::c_int) -> libc::c_int>,
    pub xRelease: Option<unsafe extern "C" fn(_: *mut sqlite3_vtab, _: libc::c_int) -> libc::c_int>,
    pub xRollbackTo:
        Option<unsafe extern "C" fn(_: *mut sqlite3_vtab, _: libc::c_int) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab {
    pub pModule: *const sqlite3_module,
    pub nRef: libc::c_int,
    pub zErrMsg: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab,
}
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type u64_0 = sqlite3_uint64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA3Context {
    pub u: C2RustUnnamed,
    pub nRate: libc::c_uint,
    pub nLoaded: libc::c_uint,
    pub ixMask: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: [u64_0; 25],
    pub x: [libc::c_uchar; 1600],
}
/*
** 2017-03-08
**
** The author disclaims copyright to this source code.  In place of
** a legal notice, here is a blessing:
**
**    May you do good and not evil.
**    May you find forgiveness for yourself and forgive others.
**    May you share freely, never taking more than you give.
**
******************************************************************************
**
** This SQLite extension implements functions that compute SHA3 hashes.
** Two SQL functions are implemented:
**
**     sha3(X,SIZE)
**     sha3_query(Y,SIZE)
**
** The sha3(X) function computes the SHA3 hash of the input X, or NULL if
** X is NULL.
**
** The sha3_query(Y) function evalutes all queries in the SQL statements of Y
** and returns a hash of their results.
**
** The SIZE argument is optional.  If omitted, the SHA3-256 hash algorithm
** is used.  If SIZE is included it must be one of the integers 224, 256,
** 384, or 512, to determine SHA3 hash variant that is computed.
*/
#[no_mangle]
pub static mut sqlite3_api: *const sqlite3_api_routines = 0 as *const sqlite3_api_routines;
/* Insert next input into u.x[nLoaded^ixMask]. */
/*
** A single step of the Keccak mixing function for a 1600-bit state
*/
unsafe extern "C" fn KeccakF1600Step(mut p: *mut SHA3Context) {
    let mut i: libc::c_int = 0;
    let mut b0: u64_0 = 0;
    let mut b1: u64_0 = 0;
    let mut b2: u64_0 = 0;
    let mut b3: u64_0 = 0;
    let mut b4: u64_0 = 0;
    let mut c0: u64_0 = 0;
    let mut c1: u64_0 = 0;
    let mut c2: u64_0 = 0;
    let mut c3: u64_0 = 0;
    let mut c4: u64_0 = 0;
    let mut d0: u64_0 = 0;
    let mut d1: u64_0 = 0;
    let mut d2: u64_0 = 0;
    let mut d3: u64_0 = 0;
    let mut d4: u64_0 = 0;
    static mut RC: [u64_0; 24] = [
        0x1u64,
        0x8082u64,
        0x800000000000808au64,
        0x8000000080008000u64,
        0x808bu64,
        0x80000001u64,
        0x8000000080008081u64,
        0x8000000000008009u64,
        0x8au64,
        0x88u64,
        0x80008009u64,
        0x8000000au64,
        0x8000808bu64,
        0x800000000000008bu64,
        0x8000000000008089u64,
        0x8000000000008003u64,
        0x8000000000008002u64,
        0x8000000000000080u64,
        0x800au64,
        0x800000008000000au64,
        0x8000000080008081u64,
        0x8000000000008080u64,
        0x80000001u64,
        0x8000000080008008u64,
    ];
    i = 0i32;
    while i < 24i32 {
        c0 = (*p).u.s[0] ^ (*p).u.s[5] ^ (*p).u.s[10] ^ (*p).u.s[15] ^ (*p).u.s[20];
        c1 = (*p).u.s[1] ^ (*p).u.s[6] ^ (*p).u.s[11] ^ (*p).u.s[16] ^ (*p).u.s[21];
        c2 = (*p).u.s[2] ^ (*p).u.s[7] ^ (*p).u.s[12] ^ (*p).u.s[17] ^ (*p).u.s[22];
        c3 = (*p).u.s[3] ^ (*p).u.s[8] ^ (*p).u.s[13] ^ (*p).u.s[18] ^ (*p).u.s[23];
        c4 = (*p).u.s[4] ^ (*p).u.s[9] ^ (*p).u.s[14] ^ (*p).u.s[19] ^ (*p).u.s[24];
        d0 = c4 ^ (c1 << 1i32 | c1 >> 64i32 - 1i32);
        d1 = c0 ^ (c2 << 1i32 | c2 >> 64i32 - 1i32);
        d2 = c1 ^ (c3 << 1i32 | c3 >> 64i32 - 1i32);
        d3 = c2 ^ (c4 << 1i32 | c4 >> 64i32 - 1i32);
        d4 = c3 ^ (c0 << 1i32 | c0 >> 64i32 - 1i32);
        b0 = (*p).u.s[0] ^ d0;
        b1 = ((*p).u.s[6] ^ d1) << 44i32 | ((*p).u.s[6] ^ d1) >> 64i32 - 44i32;
        b2 = ((*p).u.s[12] ^ d2) << 43i32 | ((*p).u.s[12] ^ d2) >> 64i32 - 43i32;
        b3 = ((*p).u.s[18] ^ d3) << 21i32 | ((*p).u.s[18] ^ d3) >> 64i32 - 21i32;
        b4 = ((*p).u.s[24] ^ d4) << 14i32 | ((*p).u.s[24] ^ d4) >> 64i32 - 14i32;
        (*p).u.s[0] = b0 ^ !b1 & b2;
        (*p).u.s[0] ^= RC[i as usize];
        (*p).u.s[6] = b1 ^ !b2 & b3;
        (*p).u.s[12] = b2 ^ !b3 & b4;
        (*p).u.s[18] = b3 ^ !b4 & b0;
        (*p).u.s[24] = b4 ^ !b0 & b1;
        b2 = ((*p).u.s[10] ^ d0) << 3i32 | ((*p).u.s[10] ^ d0) >> 64i32 - 3i32;
        b3 = ((*p).u.s[16] ^ d1) << 45i32 | ((*p).u.s[16] ^ d1) >> 64i32 - 45i32;
        b4 = ((*p).u.s[22] ^ d2) << 61i32 | ((*p).u.s[22] ^ d2) >> 64i32 - 61i32;
        b0 = ((*p).u.s[3] ^ d3) << 28i32 | ((*p).u.s[3] ^ d3) >> 64i32 - 28i32;
        b1 = ((*p).u.s[9] ^ d4) << 20i32 | ((*p).u.s[9] ^ d4) >> 64i32 - 20i32;
        (*p).u.s[10] = b0 ^ !b1 & b2;
        (*p).u.s[16] = b1 ^ !b2 & b3;
        (*p).u.s[22] = b2 ^ !b3 & b4;
        (*p).u.s[3] = b3 ^ !b4 & b0;
        (*p).u.s[9] = b4 ^ !b0 & b1;
        b4 = ((*p).u.s[20] ^ d0) << 18i32 | ((*p).u.s[20] ^ d0) >> 64i32 - 18i32;
        b0 = ((*p).u.s[1] ^ d1) << 1i32 | ((*p).u.s[1] ^ d1) >> 64i32 - 1i32;
        b1 = ((*p).u.s[7] ^ d2) << 6i32 | ((*p).u.s[7] ^ d2) >> 64i32 - 6i32;
        b2 = ((*p).u.s[13] ^ d3) << 25i32 | ((*p).u.s[13] ^ d3) >> 64i32 - 25i32;
        b3 = ((*p).u.s[19] ^ d4) << 8i32 | ((*p).u.s[19] ^ d4) >> 64i32 - 8i32;
        (*p).u.s[20] = b0 ^ !b1 & b2;
        (*p).u.s[1] = b1 ^ !b2 & b3;
        (*p).u.s[7] = b2 ^ !b3 & b4;
        (*p).u.s[13] = b3 ^ !b4 & b0;
        (*p).u.s[19] = b4 ^ !b0 & b1;
        b1 = ((*p).u.s[5] ^ d0) << 36i32 | ((*p).u.s[5] ^ d0) >> 64i32 - 36i32;
        b2 = ((*p).u.s[11] ^ d1) << 10i32 | ((*p).u.s[11] ^ d1) >> 64i32 - 10i32;
        b3 = ((*p).u.s[17] ^ d2) << 15i32 | ((*p).u.s[17] ^ d2) >> 64i32 - 15i32;
        b4 = ((*p).u.s[23] ^ d3) << 56i32 | ((*p).u.s[23] ^ d3) >> 64i32 - 56i32;
        b0 = ((*p).u.s[4] ^ d4) << 27i32 | ((*p).u.s[4] ^ d4) >> 64i32 - 27i32;
        (*p).u.s[5] = b0 ^ !b1 & b2;
        (*p).u.s[11] = b1 ^ !b2 & b3;
        (*p).u.s[17] = b2 ^ !b3 & b4;
        (*p).u.s[23] = b3 ^ !b4 & b0;
        (*p).u.s[4] = b4 ^ !b0 & b1;
        b3 = ((*p).u.s[15] ^ d0) << 41i32 | ((*p).u.s[15] ^ d0) >> 64i32 - 41i32;
        b4 = ((*p).u.s[21] ^ d1) << 2i32 | ((*p).u.s[21] ^ d1) >> 64i32 - 2i32;
        b0 = ((*p).u.s[2] ^ d2) << 62i32 | ((*p).u.s[2] ^ d2) >> 64i32 - 62i32;
        b1 = ((*p).u.s[8] ^ d3) << 55i32 | ((*p).u.s[8] ^ d3) >> 64i32 - 55i32;
        b2 = ((*p).u.s[14] ^ d4) << 39i32 | ((*p).u.s[14] ^ d4) >> 64i32 - 39i32;
        (*p).u.s[15] = b0 ^ !b1 & b2;
        (*p).u.s[21] = b1 ^ !b2 & b3;
        (*p).u.s[2] = b2 ^ !b3 & b4;
        (*p).u.s[8] = b3 ^ !b4 & b0;
        (*p).u.s[14] = b4 ^ !b0 & b1;
        c0 = (*p).u.s[0] ^ (*p).u.s[10] ^ (*p).u.s[20] ^ (*p).u.s[5] ^ (*p).u.s[15];
        c1 = (*p).u.s[6] ^ (*p).u.s[16] ^ (*p).u.s[1] ^ (*p).u.s[11] ^ (*p).u.s[21];
        c2 = (*p).u.s[12] ^ (*p).u.s[22] ^ (*p).u.s[7] ^ (*p).u.s[17] ^ (*p).u.s[2];
        c3 = (*p).u.s[18] ^ (*p).u.s[3] ^ (*p).u.s[13] ^ (*p).u.s[23] ^ (*p).u.s[8];
        c4 = (*p).u.s[24] ^ (*p).u.s[9] ^ (*p).u.s[19] ^ (*p).u.s[4] ^ (*p).u.s[14];
        d0 = c4 ^ (c1 << 1i32 | c1 >> 64i32 - 1i32);
        d1 = c0 ^ (c2 << 1i32 | c2 >> 64i32 - 1i32);
        d2 = c1 ^ (c3 << 1i32 | c3 >> 64i32 - 1i32);
        d3 = c2 ^ (c4 << 1i32 | c4 >> 64i32 - 1i32);
        d4 = c3 ^ (c0 << 1i32 | c0 >> 64i32 - 1i32);
        b0 = (*p).u.s[0] ^ d0;
        b1 = ((*p).u.s[16] ^ d1) << 44i32 | ((*p).u.s[16] ^ d1) >> 64i32 - 44i32;
        b2 = ((*p).u.s[7] ^ d2) << 43i32 | ((*p).u.s[7] ^ d2) >> 64i32 - 43i32;
        b3 = ((*p).u.s[23] ^ d3) << 21i32 | ((*p).u.s[23] ^ d3) >> 64i32 - 21i32;
        b4 = ((*p).u.s[14] ^ d4) << 14i32 | ((*p).u.s[14] ^ d4) >> 64i32 - 14i32;
        (*p).u.s[0] = b0 ^ !b1 & b2;
        (*p).u.s[0] ^= RC[(i + 1i32) as usize];
        (*p).u.s[16] = b1 ^ !b2 & b3;
        (*p).u.s[7] = b2 ^ !b3 & b4;
        (*p).u.s[23] = b3 ^ !b4 & b0;
        (*p).u.s[14] = b4 ^ !b0 & b1;
        b2 = ((*p).u.s[20] ^ d0) << 3i32 | ((*p).u.s[20] ^ d0) >> 64i32 - 3i32;
        b3 = ((*p).u.s[11] ^ d1) << 45i32 | ((*p).u.s[11] ^ d1) >> 64i32 - 45i32;
        b4 = ((*p).u.s[2] ^ d2) << 61i32 | ((*p).u.s[2] ^ d2) >> 64i32 - 61i32;
        b0 = ((*p).u.s[18] ^ d3) << 28i32 | ((*p).u.s[18] ^ d3) >> 64i32 - 28i32;
        b1 = ((*p).u.s[9] ^ d4) << 20i32 | ((*p).u.s[9] ^ d4) >> 64i32 - 20i32;
        (*p).u.s[20] = b0 ^ !b1 & b2;
        (*p).u.s[11] = b1 ^ !b2 & b3;
        (*p).u.s[2] = b2 ^ !b3 & b4;
        (*p).u.s[18] = b3 ^ !b4 & b0;
        (*p).u.s[9] = b4 ^ !b0 & b1;
        b4 = ((*p).u.s[15] ^ d0) << 18i32 | ((*p).u.s[15] ^ d0) >> 64i32 - 18i32;
        b0 = ((*p).u.s[6] ^ d1) << 1i32 | ((*p).u.s[6] ^ d1) >> 64i32 - 1i32;
        b1 = ((*p).u.s[22] ^ d2) << 6i32 | ((*p).u.s[22] ^ d2) >> 64i32 - 6i32;
        b2 = ((*p).u.s[13] ^ d3) << 25i32 | ((*p).u.s[13] ^ d3) >> 64i32 - 25i32;
        b3 = ((*p).u.s[4] ^ d4) << 8i32 | ((*p).u.s[4] ^ d4) >> 64i32 - 8i32;
        (*p).u.s[15] = b0 ^ !b1 & b2;
        (*p).u.s[6] = b1 ^ !b2 & b3;
        (*p).u.s[22] = b2 ^ !b3 & b4;
        (*p).u.s[13] = b3 ^ !b4 & b0;
        (*p).u.s[4] = b4 ^ !b0 & b1;
        b1 = ((*p).u.s[10] ^ d0) << 36i32 | ((*p).u.s[10] ^ d0) >> 64i32 - 36i32;
        b2 = ((*p).u.s[1] ^ d1) << 10i32 | ((*p).u.s[1] ^ d1) >> 64i32 - 10i32;
        b3 = ((*p).u.s[17] ^ d2) << 15i32 | ((*p).u.s[17] ^ d2) >> 64i32 - 15i32;
        b4 = ((*p).u.s[8] ^ d3) << 56i32 | ((*p).u.s[8] ^ d3) >> 64i32 - 56i32;
        b0 = ((*p).u.s[24] ^ d4) << 27i32 | ((*p).u.s[24] ^ d4) >> 64i32 - 27i32;
        (*p).u.s[10] = b0 ^ !b1 & b2;
        (*p).u.s[1] = b1 ^ !b2 & b3;
        (*p).u.s[17] = b2 ^ !b3 & b4;
        (*p).u.s[8] = b3 ^ !b4 & b0;
        (*p).u.s[24] = b4 ^ !b0 & b1;
        b3 = ((*p).u.s[5] ^ d0) << 41i32 | ((*p).u.s[5] ^ d0) >> 64i32 - 41i32;
        b4 = ((*p).u.s[21] ^ d1) << 2i32 | ((*p).u.s[21] ^ d1) >> 64i32 - 2i32;
        b0 = ((*p).u.s[12] ^ d2) << 62i32 | ((*p).u.s[12] ^ d2) >> 64i32 - 62i32;
        b1 = ((*p).u.s[3] ^ d3) << 55i32 | ((*p).u.s[3] ^ d3) >> 64i32 - 55i32;
        b2 = ((*p).u.s[19] ^ d4) << 39i32 | ((*p).u.s[19] ^ d4) >> 64i32 - 39i32;
        (*p).u.s[5] = b0 ^ !b1 & b2;
        (*p).u.s[21] = b1 ^ !b2 & b3;
        (*p).u.s[12] = b2 ^ !b3 & b4;
        (*p).u.s[3] = b3 ^ !b4 & b0;
        (*p).u.s[19] = b4 ^ !b0 & b1;
        c0 = (*p).u.s[0] ^ (*p).u.s[20] ^ (*p).u.s[15] ^ (*p).u.s[10] ^ (*p).u.s[5];
        c1 = (*p).u.s[16] ^ (*p).u.s[11] ^ (*p).u.s[6] ^ (*p).u.s[1] ^ (*p).u.s[21];
        c2 = (*p).u.s[7] ^ (*p).u.s[2] ^ (*p).u.s[22] ^ (*p).u.s[17] ^ (*p).u.s[12];
        c3 = (*p).u.s[23] ^ (*p).u.s[18] ^ (*p).u.s[13] ^ (*p).u.s[8] ^ (*p).u.s[3];
        c4 = (*p).u.s[14] ^ (*p).u.s[9] ^ (*p).u.s[4] ^ (*p).u.s[24] ^ (*p).u.s[19];
        d0 = c4 ^ (c1 << 1i32 | c1 >> 64i32 - 1i32);
        d1 = c0 ^ (c2 << 1i32 | c2 >> 64i32 - 1i32);
        d2 = c1 ^ (c3 << 1i32 | c3 >> 64i32 - 1i32);
        d3 = c2 ^ (c4 << 1i32 | c4 >> 64i32 - 1i32);
        d4 = c3 ^ (c0 << 1i32 | c0 >> 64i32 - 1i32);
        b0 = (*p).u.s[0] ^ d0;
        b1 = ((*p).u.s[11] ^ d1) << 44i32 | ((*p).u.s[11] ^ d1) >> 64i32 - 44i32;
        b2 = ((*p).u.s[22] ^ d2) << 43i32 | ((*p).u.s[22] ^ d2) >> 64i32 - 43i32;
        b3 = ((*p).u.s[8] ^ d3) << 21i32 | ((*p).u.s[8] ^ d3) >> 64i32 - 21i32;
        b4 = ((*p).u.s[19] ^ d4) << 14i32 | ((*p).u.s[19] ^ d4) >> 64i32 - 14i32;
        (*p).u.s[0] = b0 ^ !b1 & b2;
        (*p).u.s[0] ^= RC[(i + 2i32) as usize];
        (*p).u.s[11] = b1 ^ !b2 & b3;
        (*p).u.s[22] = b2 ^ !b3 & b4;
        (*p).u.s[8] = b3 ^ !b4 & b0;
        (*p).u.s[19] = b4 ^ !b0 & b1;
        b2 = ((*p).u.s[15] ^ d0) << 3i32 | ((*p).u.s[15] ^ d0) >> 64i32 - 3i32;
        b3 = ((*p).u.s[1] ^ d1) << 45i32 | ((*p).u.s[1] ^ d1) >> 64i32 - 45i32;
        b4 = ((*p).u.s[12] ^ d2) << 61i32 | ((*p).u.s[12] ^ d2) >> 64i32 - 61i32;
        b0 = ((*p).u.s[23] ^ d3) << 28i32 | ((*p).u.s[23] ^ d3) >> 64i32 - 28i32;
        b1 = ((*p).u.s[9] ^ d4) << 20i32 | ((*p).u.s[9] ^ d4) >> 64i32 - 20i32;
        (*p).u.s[15] = b0 ^ !b1 & b2;
        (*p).u.s[1] = b1 ^ !b2 & b3;
        (*p).u.s[12] = b2 ^ !b3 & b4;
        (*p).u.s[23] = b3 ^ !b4 & b0;
        (*p).u.s[9] = b4 ^ !b0 & b1;
        b4 = ((*p).u.s[5] ^ d0) << 18i32 | ((*p).u.s[5] ^ d0) >> 64i32 - 18i32;
        b0 = ((*p).u.s[16] ^ d1) << 1i32 | ((*p).u.s[16] ^ d1) >> 64i32 - 1i32;
        b1 = ((*p).u.s[2] ^ d2) << 6i32 | ((*p).u.s[2] ^ d2) >> 64i32 - 6i32;
        b2 = ((*p).u.s[13] ^ d3) << 25i32 | ((*p).u.s[13] ^ d3) >> 64i32 - 25i32;
        b3 = ((*p).u.s[24] ^ d4) << 8i32 | ((*p).u.s[24] ^ d4) >> 64i32 - 8i32;
        (*p).u.s[5] = b0 ^ !b1 & b2;
        (*p).u.s[16] = b1 ^ !b2 & b3;
        (*p).u.s[2] = b2 ^ !b3 & b4;
        (*p).u.s[13] = b3 ^ !b4 & b0;
        (*p).u.s[24] = b4 ^ !b0 & b1;
        b1 = ((*p).u.s[20] ^ d0) << 36i32 | ((*p).u.s[20] ^ d0) >> 64i32 - 36i32;
        b2 = ((*p).u.s[6] ^ d1) << 10i32 | ((*p).u.s[6] ^ d1) >> 64i32 - 10i32;
        b3 = ((*p).u.s[17] ^ d2) << 15i32 | ((*p).u.s[17] ^ d2) >> 64i32 - 15i32;
        b4 = ((*p).u.s[3] ^ d3) << 56i32 | ((*p).u.s[3] ^ d3) >> 64i32 - 56i32;
        b0 = ((*p).u.s[14] ^ d4) << 27i32 | ((*p).u.s[14] ^ d4) >> 64i32 - 27i32;
        (*p).u.s[20] = b0 ^ !b1 & b2;
        (*p).u.s[6] = b1 ^ !b2 & b3;
        (*p).u.s[17] = b2 ^ !b3 & b4;
        (*p).u.s[3] = b3 ^ !b4 & b0;
        (*p).u.s[14] = b4 ^ !b0 & b1;
        b3 = ((*p).u.s[10] ^ d0) << 41i32 | ((*p).u.s[10] ^ d0) >> 64i32 - 41i32;
        b4 = ((*p).u.s[21] ^ d1) << 2i32 | ((*p).u.s[21] ^ d1) >> 64i32 - 2i32;
        b0 = ((*p).u.s[7] ^ d2) << 62i32 | ((*p).u.s[7] ^ d2) >> 64i32 - 62i32;
        b1 = ((*p).u.s[18] ^ d3) << 55i32 | ((*p).u.s[18] ^ d3) >> 64i32 - 55i32;
        b2 = ((*p).u.s[4] ^ d4) << 39i32 | ((*p).u.s[4] ^ d4) >> 64i32 - 39i32;
        (*p).u.s[10] = b0 ^ !b1 & b2;
        (*p).u.s[21] = b1 ^ !b2 & b3;
        (*p).u.s[7] = b2 ^ !b3 & b4;
        (*p).u.s[18] = b3 ^ !b4 & b0;
        (*p).u.s[4] = b4 ^ !b0 & b1;
        c0 = (*p).u.s[0] ^ (*p).u.s[15] ^ (*p).u.s[5] ^ (*p).u.s[20] ^ (*p).u.s[10];
        c1 = (*p).u.s[11] ^ (*p).u.s[1] ^ (*p).u.s[16] ^ (*p).u.s[6] ^ (*p).u.s[21];
        c2 = (*p).u.s[22] ^ (*p).u.s[12] ^ (*p).u.s[2] ^ (*p).u.s[17] ^ (*p).u.s[7];
        c3 = (*p).u.s[8] ^ (*p).u.s[23] ^ (*p).u.s[13] ^ (*p).u.s[3] ^ (*p).u.s[18];
        c4 = (*p).u.s[19] ^ (*p).u.s[9] ^ (*p).u.s[24] ^ (*p).u.s[14] ^ (*p).u.s[4];
        d0 = c4 ^ (c1 << 1i32 | c1 >> 64i32 - 1i32);
        d1 = c0 ^ (c2 << 1i32 | c2 >> 64i32 - 1i32);
        d2 = c1 ^ (c3 << 1i32 | c3 >> 64i32 - 1i32);
        d3 = c2 ^ (c4 << 1i32 | c4 >> 64i32 - 1i32);
        d4 = c3 ^ (c0 << 1i32 | c0 >> 64i32 - 1i32);
        b0 = (*p).u.s[0] ^ d0;
        b1 = ((*p).u.s[1] ^ d1) << 44i32 | ((*p).u.s[1] ^ d1) >> 64i32 - 44i32;
        b2 = ((*p).u.s[2] ^ d2) << 43i32 | ((*p).u.s[2] ^ d2) >> 64i32 - 43i32;
        b3 = ((*p).u.s[3] ^ d3) << 21i32 | ((*p).u.s[3] ^ d3) >> 64i32 - 21i32;
        b4 = ((*p).u.s[4] ^ d4) << 14i32 | ((*p).u.s[4] ^ d4) >> 64i32 - 14i32;
        (*p).u.s[0] = b0 ^ !b1 & b2;
        (*p).u.s[0] ^= RC[(i + 3i32) as usize];
        (*p).u.s[1] = b1 ^ !b2 & b3;
        (*p).u.s[2] = b2 ^ !b3 & b4;
        (*p).u.s[3] = b3 ^ !b4 & b0;
        (*p).u.s[4] = b4 ^ !b0 & b1;
        b2 = ((*p).u.s[5] ^ d0) << 3i32 | ((*p).u.s[5] ^ d0) >> 64i32 - 3i32;
        b3 = ((*p).u.s[6] ^ d1) << 45i32 | ((*p).u.s[6] ^ d1) >> 64i32 - 45i32;
        b4 = ((*p).u.s[7] ^ d2) << 61i32 | ((*p).u.s[7] ^ d2) >> 64i32 - 61i32;
        b0 = ((*p).u.s[8] ^ d3) << 28i32 | ((*p).u.s[8] ^ d3) >> 64i32 - 28i32;
        b1 = ((*p).u.s[9] ^ d4) << 20i32 | ((*p).u.s[9] ^ d4) >> 64i32 - 20i32;
        (*p).u.s[5] = b0 ^ !b1 & b2;
        (*p).u.s[6] = b1 ^ !b2 & b3;
        (*p).u.s[7] = b2 ^ !b3 & b4;
        (*p).u.s[8] = b3 ^ !b4 & b0;
        (*p).u.s[9] = b4 ^ !b0 & b1;
        b4 = ((*p).u.s[10] ^ d0) << 18i32 | ((*p).u.s[10] ^ d0) >> 64i32 - 18i32;
        b0 = ((*p).u.s[11] ^ d1) << 1i32 | ((*p).u.s[11] ^ d1) >> 64i32 - 1i32;
        b1 = ((*p).u.s[12] ^ d2) << 6i32 | ((*p).u.s[12] ^ d2) >> 64i32 - 6i32;
        b2 = ((*p).u.s[13] ^ d3) << 25i32 | ((*p).u.s[13] ^ d3) >> 64i32 - 25i32;
        b3 = ((*p).u.s[14] ^ d4) << 8i32 | ((*p).u.s[14] ^ d4) >> 64i32 - 8i32;
        (*p).u.s[10] = b0 ^ !b1 & b2;
        (*p).u.s[11] = b1 ^ !b2 & b3;
        (*p).u.s[12] = b2 ^ !b3 & b4;
        (*p).u.s[13] = b3 ^ !b4 & b0;
        (*p).u.s[14] = b4 ^ !b0 & b1;
        b1 = ((*p).u.s[15] ^ d0) << 36i32 | ((*p).u.s[15] ^ d0) >> 64i32 - 36i32;
        b2 = ((*p).u.s[16] ^ d1) << 10i32 | ((*p).u.s[16] ^ d1) >> 64i32 - 10i32;
        b3 = ((*p).u.s[17] ^ d2) << 15i32 | ((*p).u.s[17] ^ d2) >> 64i32 - 15i32;
        b4 = ((*p).u.s[18] ^ d3) << 56i32 | ((*p).u.s[18] ^ d3) >> 64i32 - 56i32;
        b0 = ((*p).u.s[19] ^ d4) << 27i32 | ((*p).u.s[19] ^ d4) >> 64i32 - 27i32;
        (*p).u.s[15] = b0 ^ !b1 & b2;
        (*p).u.s[16] = b1 ^ !b2 & b3;
        (*p).u.s[17] = b2 ^ !b3 & b4;
        (*p).u.s[18] = b3 ^ !b4 & b0;
        (*p).u.s[19] = b4 ^ !b0 & b1;
        b3 = ((*p).u.s[20] ^ d0) << 41i32 | ((*p).u.s[20] ^ d0) >> 64i32 - 41i32;
        b4 = ((*p).u.s[21] ^ d1) << 2i32 | ((*p).u.s[21] ^ d1) >> 64i32 - 2i32;
        b0 = ((*p).u.s[22] ^ d2) << 62i32 | ((*p).u.s[22] ^ d2) >> 64i32 - 62i32;
        b1 = ((*p).u.s[23] ^ d3) << 55i32 | ((*p).u.s[23] ^ d3) >> 64i32 - 55i32;
        b2 = ((*p).u.s[24] ^ d4) << 39i32 | ((*p).u.s[24] ^ d4) >> 64i32 - 39i32;
        (*p).u.s[20] = b0 ^ !b1 & b2;
        (*p).u.s[21] = b1 ^ !b2 & b3;
        (*p).u.s[22] = b2 ^ !b3 & b4;
        (*p).u.s[23] = b3 ^ !b4 & b0;
        (*p).u.s[24] = b4 ^ !b0 & b1;
        i += 4i32
    }
}
/*
** Initialize a new hash.  iSize determines the size of the hash
** in bits and should be one of 224, 256, 384, or 512.  Or iSize
** can be zero to use the default hash size of 256 bits.
*/
unsafe extern "C" fn SHA3Init(mut p: *mut SHA3Context, mut iSize: libc::c_int) {
    memset(
        p as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<SHA3Context>() as libc::c_ulong,
    );
    if iSize >= 128i32 && iSize <= 512i32 {
        (*p).nRate = ((1600i32 - (iSize + 31i32 & !31i32) * 2i32) / 8i32) as libc::c_uint
    } else {
        (*p).nRate = ((1600i32 - 2i32 * 256i32) / 8i32) as libc::c_uint
    };
    /* Known to be little-endian at compile-time. No-op */
}
/*
** Make consecutive calls to the SHA3Update function to add new content
** to the hash
*/
unsafe extern "C" fn SHA3Update(
    mut p: *mut SHA3Context,
    mut aData: *const libc::c_uchar,
    mut nData: libc::c_uint,
) {
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    if (*p).nLoaded.wrapping_rem(8i32 as libc::c_uint) == 0i32 as libc::c_uint
        && aData.wrapping_offset_from(0 as *const libc::c_uchar) as libc::c_long
            & 7i32 as libc::c_long
            == 0i32 as libc::c_long
    {
        while i.wrapping_add(7i32 as libc::c_uint) < nData {
            (*p).u.s[(*p).nLoaded.wrapping_div(8i32 as libc::c_uint) as usize] ^=
                *(&*aData.offset(i as isize) as *const libc::c_uchar as *mut u64_0);
            (*p).nLoaded = (*p).nLoaded.wrapping_add(8i32 as libc::c_uint);
            if (*p).nLoaded >= (*p).nRate {
                KeccakF1600Step(p);
                (*p).nLoaded = 0i32 as libc::c_uint
            }
            i = i.wrapping_add(8i32 as libc::c_uint)
        }
    }
    while i < nData {
        (*p).u.x[(*p).nLoaded as usize] = ((*p).u.x[(*p).nLoaded as usize] as libc::c_int
            ^ *aData.offset(i as isize) as libc::c_int)
            as libc::c_uchar;
        (*p).nLoaded = (*p).nLoaded.wrapping_add(1);
        if (*p).nLoaded == (*p).nRate {
            KeccakF1600Step(p);
            (*p).nLoaded = 0i32 as libc::c_uint
        }
        i = i.wrapping_add(1)
    }
}
/*
** After all content has been added, invoke SHA3Final() to compute
** the final hash.  The function returns a pointer to the binary
** hash value.
*/
unsafe extern "C" fn SHA3Final(mut p: *mut SHA3Context) -> *mut libc::c_uchar {
    let mut i: libc::c_uint = 0;
    if (*p).nLoaded == (*p).nRate.wrapping_sub(1i32 as libc::c_uint) {
        let c1: libc::c_uchar = 0x86i32 as libc::c_uchar;
        SHA3Update(p, &c1, 1i32 as libc::c_uint);
    } else {
        let c2: libc::c_uchar = 0x6i32 as libc::c_uchar;
        let c3: libc::c_uchar = 0x80i32 as libc::c_uchar;
        SHA3Update(p, &c2, 1i32 as libc::c_uint);
        (*p).nLoaded = (*p).nRate.wrapping_sub(1i32 as libc::c_uint);
        SHA3Update(p, &c3, 1i32 as libc::c_uint);
    }
    i = 0i32 as libc::c_uint;
    while i < (*p).nRate {
        (*p).u.x[i.wrapping_add((*p).nRate) as usize] = (*p).u.x[(i ^ (*p).ixMask) as usize];
        i = i.wrapping_add(1)
    }
    return &mut *(*p).u.x.as_mut_ptr().offset((*p).nRate as isize) as *mut libc::c_uchar;
}
/* End of the hashing logic
*****************************************************************************/
/*
** Implementation of the sha3(X,SIZE) function.
**
** Return a BLOB which is the SIZE-bit SHA3 hash of X.  The default
** size is 256.  If X is a BLOB, it is hashed as is.
** For all other non-NULL types of input, X is converted into a UTF-8 string
** and the string is hashed without the trailing 0x00 terminator.  The hash
** of a NULL value is NULL.
*/
unsafe extern "C" fn sha3Func(
    mut context: *mut sqlite3_context,
    mut argc: libc::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut cx: SHA3Context = SHA3Context {
        u: C2RustUnnamed { s: [0; 25] },
        nRate: 0,
        nLoaded: 0,
        ixMask: 0,
    };
    let mut eType: libc::c_int = (*sqlite3_api)
        .value_type
        .expect("non-null function pointer")(*argv.offset(0));
    let mut nByte: libc::c_int = (*sqlite3_api)
        .value_bytes
        .expect("non-null function pointer")(*argv.offset(0));
    let mut iSize: libc::c_int = 0;
    if argc == 1i32 {
        iSize = 256i32
    } else {
        iSize = (*sqlite3_api).value_int.expect("non-null function pointer")(*argv.offset(1));
        if iSize != 224i32 && iSize != 256i32 && iSize != 384i32 && iSize != 512i32 {
            (*sqlite3_api)
                .result_error
                .expect("non-null function pointer")(
                context,
                b"SHA3 size should be one of: 224 256 384 512\x00" as *const u8
                    as *const libc::c_char,
                -1i32,
            );
            return;
        }
    }
    if eType == 5i32 {
        return;
    }
    SHA3Init(&mut cx, iSize);
    if eType == 4i32 {
        SHA3Update(
            &mut cx,
            (*sqlite3_api)
                .value_blob
                .expect("non-null function pointer")(*argv.offset(0))
                as *const libc::c_uchar,
            nByte as libc::c_uint,
        );
    } else {
        SHA3Update(
            &mut cx,
            (*sqlite3_api)
                .value_text
                .expect("non-null function pointer")(*argv.offset(0)),
            nByte as libc::c_uint,
        );
    }
    (*sqlite3_api)
        .result_blob
        .expect("non-null function pointer")(
        context,
        SHA3Final(&mut cx) as *const libc::c_void,
        iSize / 8i32,
        ::std::mem::transmute::<libc::intptr_t, sqlite3_destructor_type>(-1i32 as libc::intptr_t),
    );
}
/* Compute a string using sqlite3_vsnprintf() with a maximum length
** of 50 bytes and add it to the hash.
*/
unsafe extern "C" fn hash_step_vformat(
    mut p: *mut SHA3Context,
    mut zFormat: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut n: libc::c_int = 0;
    let mut zBuf: [libc::c_char; 50] = [0; 50];
    ap = args.clone();
    (*sqlite3_api)
        .xvsnprintf
        .expect("non-null function pointer")(
        ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong as libc::c_int,
        zBuf.as_mut_ptr(),
        zFormat,
        ap.as_va_list(),
    );
    n = strlen(zBuf.as_mut_ptr()) as libc::c_int;
    SHA3Update(
        p,
        zBuf.as_mut_ptr() as *mut libc::c_uchar,
        n as libc::c_uint,
    );
}
/*
** Implementation of the sha3_query(SQL,SIZE) function.
**
** This function compiles and runs the SQL statement(s) given in the
** argument. The results are hashed using a SIZE-bit SHA3.  The default
** size is 256.
**
** The format of the byte stream that is hashed is summarized as follows:
**
**       S<n>:<sql>
**       R
**       N
**       I<int>
**       F<ieee-float>
**       B<size>:<bytes>
**       T<size>:<text>
**
** <sql> is the original SQL text for each statement run and <n> is
** the size of that text.  The SQL text is UTF-8.  A single R character
** occurs before the start of each row.  N means a NULL value.
** I mean an 8-byte little-endian integer <int>.  F is a floating point
** number with an 8-byte little-endian IEEE floating point value <ieee-float>.
** B means blobs of <size> bytes.  T means text rendered as <size>
** bytes of UTF-8.  The <n> and <size> values are expressed as an ASCII
** text integers.
**
** For each SQL statement in the X input, there is one S segment.  Each
** S segment is followed by zero or more R segments, one for each row in the
** result set.  After each R, there are one or more N, I, F, B, or T segments,
** one for each column in the result set.  Segments are concatentated directly
** with no delimiters of any kind.
*/
unsafe extern "C" fn sha3QueryFunc(
    mut context: *mut sqlite3_context,
    mut argc: libc::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut db: *mut sqlite3 = (*sqlite3_api)
        .context_db_handle
        .expect("non-null function pointer")(context); /* Number of columns in the result set */
    let mut zSql: *const libc::c_char = (*sqlite3_api)
        .value_text
        .expect("non-null function pointer")(*argv.offset(0))
        as *const libc::c_char; /* Loop counter */
    let mut pStmt: *mut sqlite3_stmt = 0 as *mut sqlite3_stmt;
    let mut nCol: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    let mut cx: SHA3Context = SHA3Context {
        u: C2RustUnnamed { s: [0; 25] },
        nRate: 0,
        nLoaded: 0,
        ixMask: 0,
    };
    let mut iSize: libc::c_int = 0;
    if argc == 1i32 {
        iSize = 256i32
    } else {
        iSize = (*sqlite3_api).value_int.expect("non-null function pointer")(*argv.offset(1));
        if iSize != 224i32 && iSize != 256i32 && iSize != 384i32 && iSize != 512i32 {
            (*sqlite3_api)
                .result_error
                .expect("non-null function pointer")(
                context,
                b"SHA3 size should be one of: 224 256 384 512\x00" as *const u8
                    as *const libc::c_char,
                -1i32,
            );
            return;
        }
    }
    if zSql.is_null() {
        return;
    }
    SHA3Init(&mut cx, iSize);
    while *zSql.offset(0) != 0 {
        rc = (*sqlite3_api)
            .prepare_v2
            .expect("non-null function pointer")(
            db, zSql, -1i32, &mut pStmt, &mut zSql
        );
        if rc != 0 {
            let mut zMsg: *mut libc::c_char =
                (*sqlite3_api).mprintf.expect("non-null function pointer")(
                    b"error SQL statement [%s]: %s\x00" as *const u8 as *const libc::c_char,
                    zSql,
                    (*sqlite3_api).errmsg.expect("non-null function pointer")(db),
                );
            (*sqlite3_api).finalize.expect("non-null function pointer")(pStmt);
            (*sqlite3_api)
                .result_error
                .expect("non-null function pointer")(context, zMsg, -1i32);
            (*sqlite3_api).free.expect("non-null function pointer")(zMsg as *mut libc::c_void);
            return;
        }
        if (*sqlite3_api)
            .stmt_readonly
            .expect("non-null function pointer")(pStmt)
            == 0
        {
            let mut zMsg_0: *mut libc::c_char =
                (*sqlite3_api).mprintf.expect("non-null function pointer")(
                    b"non-query: [%s]\x00" as *const u8 as *const libc::c_char,
                    (*sqlite3_api).sql.expect("non-null function pointer")(pStmt),
                );
            (*sqlite3_api).finalize.expect("non-null function pointer")(pStmt);
            (*sqlite3_api)
                .result_error
                .expect("non-null function pointer")(context, zMsg_0, -1i32);
            (*sqlite3_api).free.expect("non-null function pointer")(zMsg_0 as *mut libc::c_void);
            return;
        }
        nCol = (*sqlite3_api)
            .column_count
            .expect("non-null function pointer")(pStmt);
        z = (*sqlite3_api).sql.expect("non-null function pointer")(pStmt);
        n = strlen(z) as libc::c_int;
        hash_step_vformat(
            &mut cx as *mut SHA3Context,
            b"S%d:\x00" as *const u8 as *const libc::c_char,
            n,
        );
        SHA3Update(&mut cx, z as *mut libc::c_uchar, n as libc::c_uint);
        /* Compute a hash over the result of the query */
        while 100i32 == (*sqlite3_api).step.expect("non-null function pointer")(pStmt) {
            SHA3Update(
                &mut cx,
                b"R\x00" as *const u8 as *const libc::c_char as *const libc::c_uchar,
                1i32 as libc::c_uint,
            );
            i = 0i32;
            while i < nCol {
                match (*sqlite3_api)
                    .column_type
                    .expect("non-null function pointer")(pStmt, i)
                {
                    5 => {
                        SHA3Update(
                            &mut cx,
                            b"N\x00" as *const u8 as *const libc::c_char as *const libc::c_uchar,
                            1i32 as libc::c_uint,
                        );
                    }
                    1 => {
                        let mut u: sqlite3_uint64 = 0;
                        let mut j: libc::c_int = 0;
                        let mut x: [libc::c_uchar; 9] = [0; 9];
                        let mut v: sqlite3_int64 = (*sqlite3_api)
                            .column_int64
                            .expect("non-null function pointer")(
                            pStmt, i
                        );
                        memcpy(
                            &mut u as *mut sqlite3_uint64 as *mut libc::c_void,
                            &mut v as *mut sqlite3_int64 as *const libc::c_void,
                            8i32 as libc::c_ulong,
                        );
                        j = 8i32;
                        while j >= 1i32 {
                            x[j as usize] = (u & 0xffi32 as libc::c_ulonglong) as libc::c_uchar;
                            u >>= 8i32;
                            j -= 1
                        }
                        x[0] = 'I' as i32 as libc::c_uchar;
                        SHA3Update(&mut cx, x.as_mut_ptr(), 9i32 as libc::c_uint);
                    }
                    2 => {
                        let mut u_0: sqlite3_uint64 = 0;
                        let mut j_0: libc::c_int = 0;
                        let mut x_0: [libc::c_uchar; 9] = [0; 9];
                        let mut r: libc::c_double = (*sqlite3_api)
                            .column_double
                            .expect("non-null function pointer")(
                            pStmt, i
                        );
                        memcpy(
                            &mut u_0 as *mut sqlite3_uint64 as *mut libc::c_void,
                            &mut r as *mut libc::c_double as *const libc::c_void,
                            8i32 as libc::c_ulong,
                        );
                        j_0 = 8i32;
                        while j_0 >= 1i32 {
                            x_0[j_0 as usize] =
                                (u_0 & 0xffi32 as libc::c_ulonglong) as libc::c_uchar;
                            u_0 >>= 8i32;
                            j_0 -= 1
                        }
                        x_0[0] = 'F' as i32 as libc::c_uchar;
                        SHA3Update(&mut cx, x_0.as_mut_ptr(), 9i32 as libc::c_uint);
                    }
                    3 => {
                        let mut n2: libc::c_int = (*sqlite3_api)
                            .column_bytes
                            .expect("non-null function pointer")(
                            pStmt, i
                        );
                        let mut z2: *const libc::c_uchar = (*sqlite3_api)
                            .column_text
                            .expect("non-null function pointer")(
                            pStmt, i
                        );
                        hash_step_vformat(
                            &mut cx as *mut SHA3Context,
                            b"T%d:\x00" as *const u8 as *const libc::c_char,
                            n2,
                        );
                        SHA3Update(&mut cx, z2, n2 as libc::c_uint);
                    }
                    4 => {
                        let mut n2_0: libc::c_int = (*sqlite3_api)
                            .column_bytes
                            .expect("non-null function pointer")(
                            pStmt, i
                        );
                        let mut z2_0: *const libc::c_uchar = (*sqlite3_api)
                            .column_blob
                            .expect("non-null function pointer")(
                            pStmt, i
                        )
                            as *const libc::c_uchar;
                        hash_step_vformat(
                            &mut cx as *mut SHA3Context,
                            b"B%d:\x00" as *const u8 as *const libc::c_char,
                            n2_0,
                        );
                        SHA3Update(&mut cx, z2_0, n2_0 as libc::c_uint);
                    }
                    _ => {}
                }
                i += 1
            }
        }
        (*sqlite3_api).finalize.expect("non-null function pointer")(pStmt);
    }
    (*sqlite3_api)
        .result_blob
        .expect("non-null function pointer")(
        context,
        SHA3Final(&mut cx) as *const libc::c_void,
        iSize / 8i32,
        ::std::mem::transmute::<libc::intptr_t, sqlite3_destructor_type>(-1i32 as libc::intptr_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_shathree_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut libc::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> libc::c_int {
    let mut rc: libc::c_int = 0i32;
    sqlite3_api = pApi;
    /* Unused parameter */
    rc = (*sqlite3_api)
        .create_function
        .expect("non-null function pointer")(
        db,
        b"sha3\x00" as *const u8 as *const libc::c_char,
        1i32,
        1i32,
        0 as *mut libc::c_void,
        Some(
            sha3Func
                as unsafe extern "C" fn(
                    _: *mut sqlite3_context,
                    _: libc::c_int,
                    _: *mut *mut sqlite3_value,
                ) -> (),
        ),
        None,
        None,
    );
    if rc == 0i32 {
        rc = (*sqlite3_api)
            .create_function
            .expect("non-null function pointer")(
            db,
            b"sha3\x00" as *const u8 as *const libc::c_char,
            2i32,
            1i32,
            0 as *mut libc::c_void,
            Some(
                sha3Func
                    as unsafe extern "C" fn(
                        _: *mut sqlite3_context,
                        _: libc::c_int,
                        _: *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        )
    }
    if rc == 0i32 {
        rc = (*sqlite3_api)
            .create_function
            .expect("non-null function pointer")(
            db,
            b"sha3_query\x00" as *const u8 as *const libc::c_char,
            1i32,
            1i32,
            0 as *mut libc::c_void,
            Some(
                sha3QueryFunc
                    as unsafe extern "C" fn(
                        _: *mut sqlite3_context,
                        _: libc::c_int,
                        _: *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        )
    }
    if rc == 0i32 {
        rc = (*sqlite3_api)
            .create_function
            .expect("non-null function pointer")(
            db,
            b"sha3_query\x00" as *const u8 as *const libc::c_char,
            2i32,
            1i32,
            0 as *mut libc::c_void,
            Some(
                sha3QueryFunc
                    as unsafe extern "C" fn(
                        _: *mut sqlite3_context,
                        _: libc::c_int,
                        _: *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        )
    }
    return rc;
}

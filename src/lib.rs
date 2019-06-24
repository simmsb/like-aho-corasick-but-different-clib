use std::boxed::Box;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

use like_aho_corasick_but_different::SimpleFinder;

#[repr(C)]
pub struct Searcher {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SearchElement {
    key: *const c_char,
    val: *const c_void,
}

#[repr(C)]
pub struct SearchResult {
    values: *const *const c_void,
    length: usize,
}

#[no_mangle]
pub extern "C" fn new_searcher(
    search_strings: *const SearchElement,
    num_strings: usize,
) -> *const Searcher {
    let s = unsafe { std::slice::from_raw_parts(search_strings, num_strings) };
    let search_strings: Vec<_> = s
        .into_iter()
        .filter_map(|s| {
            let cstr = unsafe { CStr::from_ptr(s.key) };
            Some((cstr.to_str().ok()?, s.val))
        })
        .collect();

    let searcher = Box::new(SimpleFinder::new(search_strings));
    Box::into_raw(searcher) as *const Searcher
}

#[no_mangle]
pub extern "C" fn search_searcher(
    searcher: *const Searcher,
    haystack: *const c_char,
) -> SearchResult {
    let searcher = unsafe { Box::from_raw(searcher as *mut SimpleFinder<*const c_void>) };
    let haystack = unsafe { CStr::from_ptr(haystack).to_str().unwrap() };

    let found: Vec<_> = searcher.find_all_unique(haystack).into_iter().collect();
    let found = found.into_boxed_slice();

    let result = SearchResult {
        values: found.as_ptr() as *const *const c_void,
        length: found.len(),
    };
    std::mem::forget(found);
    std::mem::forget(searcher);
    result
}

#[no_mangle]
pub extern "C" fn deallocate_result(result: SearchResult) {
    let results = unsafe { std::slice::from_raw_parts(result.values, result.length) };
    drop(results);
}

#[no_mangle]
pub extern "C" fn deallocate_searcher(searcher: *mut Searcher) {
    let searcher = unsafe { Box::from_raw(searcher as *mut SimpleFinder<*const c_void>) };
    drop(searcher);
}

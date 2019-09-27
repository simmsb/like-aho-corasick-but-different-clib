use std::boxed::Box;
use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::os::raw::{c_char, c_void};

use lacbd::SimpleFinder;

#[repr(C)]
pub struct Searcher {
    _private: [u8; 0],
}

impl Searcher {
    unsafe fn from_raw(searcher: *const Self) -> ManuallyDrop<Box<SimpleFinder<*const c_void>>> {
        let searcher = Box::from_raw(searcher as *mut SimpleFinder<*const c_void>);
        ManuallyDrop::new(searcher)
    }
}

#[repr(C)]
pub struct SearchElement {
    key: *const c_char,
    val: *const c_void,
}

#[repr(C)]
pub struct ExtendedResultElement {
    value: *const c_void,
    start: usize,
    end: usize,
}

#[repr(C)]
pub struct SearchResult {
    values: *const *const c_void,
    length: usize,
}

#[repr(C)]
pub struct ExtendedSearchResult {
    values: *const ExtendedResultElement,
    length: usize,
}

#[no_mangle]
pub extern "C" fn new_searcher(
    search_strings: *const SearchElement,
    num_strings: usize,
) -> *const Searcher {
    let s = unsafe { std::slice::from_raw_parts(search_strings, num_strings) };
    let search_strings: Vec<_> = s
        .iter()
        .filter_map(|s| {
            let cstr = unsafe { CStr::from_ptr(s.key) };
            Some((cstr.to_str().ok()?, s.val))
        })
        .collect();

    let searcher = Box::new(SimpleFinder::new(search_strings));
    Box::into_raw(searcher) as *const Searcher
}

#[no_mangle]
pub extern "C" fn searcher_size(searcher: *const Searcher) -> usize {
    let searcher = unsafe { Searcher::from_raw(searcher) };
    searcher.heap_bytes()
}

#[no_mangle]
pub extern "C" fn searcher_pattern_count(searcher: *const Searcher) -> usize {
    let searcher = unsafe { Searcher::from_raw(searcher) };
    searcher.pattern_count()
}

#[no_mangle]
pub extern "C" fn search_searcher(
    searcher: *const Searcher,
    haystack: *const c_char,
) -> SearchResult {
    let searcher = unsafe { Searcher::from_raw(searcher) };
    let haystack = unsafe { CStr::from_ptr(haystack).to_str().unwrap() };

    let found: Vec<_> = searcher.find_all_unique(haystack).into_iter().collect();
    let found = found.into_boxed_slice();

    let result = SearchResult {
        values: found.as_ptr() as *const *const c_void,
        length: found.len(),
    };
    std::mem::forget(found);
    result
}

#[no_mangle]
pub extern "C" fn search_searcher_extended(
    searcher: *const Searcher,
    haystack: *const c_char,
) -> ExtendedSearchResult {
    let searcher = unsafe { Searcher::from_raw(searcher) };
    let haystack = unsafe { CStr::from_ptr(haystack).to_str().unwrap() };

    let found: Vec<_> = searcher
        .find_all(haystack)
        .map(|(match_, value)| ExtendedResultElement {
            value: *value,
            start: match_.start(),
            end: match_.end(),
        })
        .collect();
    let found = found.into_boxed_slice();

    let result = ExtendedSearchResult {
        values: found.as_ptr() as *const ExtendedResultElement,
        length: found.len(),
    };
    std::mem::forget(found);
    result
}

#[no_mangle]
pub extern "C" fn deallocate_result(result: SearchResult) {
    let results = unsafe {
        Vec::from_raw_parts(
            result.values as *mut *const c_void,
            result.length,
            result.length,
        )
    };
    drop(results)
}

#[no_mangle]
pub extern "C" fn deallocate_extended_result(result: ExtendedSearchResult) {
    let results = unsafe {
        Vec::from_raw_parts(
            result.values as *mut ExtendedResultElement,
            result.length,
            result.length,
        )
    };
    drop(results)
}

#[no_mangle]
pub extern "C" fn deallocate_searcher(searcher: *mut Searcher) {
    let mut searcher = unsafe { Searcher::from_raw(searcher) };
    unsafe { ManuallyDrop::drop(&mut searcher) };
}

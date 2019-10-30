#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn global_dist(target: &[u8], query: &[u8]) -> u32 {
    unsafe {
        let config = edlibNewAlignConfig(
            -1,
            EdlibAlignMode_EDLIB_MODE_NW,
            EdlibAlignTask_EDLIB_TASK_DISTANCE,
            std::ptr::null_mut(),
            0,
        );
        let align = edlibAlign(
            query.as_ptr() as *const i8,
            query.len() as i32,
            target.as_ptr() as *const i8,
            target.len() as i32,
            config,
        );
        let result = align.editDistance as u32;
        edlibFreeAlignResult(align);
        result
    }
}

/// Calculate the global alignment for given query and reference.
/// 0 for match, 1 for insertion to target, 2 for insertion to query, and 3 for mismatch.
pub fn global(target: &[u8], query: &[u8]) -> Vec<u8> {
    unsafe {
        let config = edlibNewAlignConfig(
            -1,
            EdlibAlignMode_EDLIB_MODE_NW,
            EdlibAlignTask_EDLIB_TASK_PATH,
            std::ptr::null_mut(),
            0,
        );
        let align = edlibAlign(
            query.as_ptr() as *const i8,
            query.len() as i32,
            target.as_ptr() as *const i8,
            target.len() as i32,
            config,
        );
        let result =
            std::slice::from_raw_parts(align.alignment, align.alignmentLength as usize).to_vec();
        edlibFreeAlignResult(align);
        result
    }
}

pub fn align(query: &[u8], refr: &[u8]) -> String {
    unsafe {
        let config = edlibNewAlignConfig(
            -1,
            EdlibAlignMode_EDLIB_MODE_SHW,
            EdlibAlignTask_EDLIB_TASK_PATH,
            std::ptr::null_mut(),
            0,
        );
        let align = edlibAlign(
            query.as_ptr() as *const i8,
            query.len() as i32,
            refr.as_ptr() as *const i8,
            refr.len() as i32,
            config,
        );
        let cigar = edlibAlignmentToCigar(
            align.alignment,
            align.alignmentLength,
            EdlibCigarFormat_EDLIB_CIGAR_EXTENDED,
        );
        edlibFreeAlignResult(align);
        let cigar = std::ffi::CString::from_raw(cigar);
        cigar.to_string_lossy().to_string()
    }
}

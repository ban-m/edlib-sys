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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug, Clone)]
pub struct Alignment {
    pub dist: u32,
    /// Pair of optimal alignments.
    pub locations: Option<Vec<(usize, usize)>>,
    /// Alignment operations.0=>Match, 1=> Ins, 2=>Del 3=> Mism
    /// Note that leading/trailing sequence in the query sequence
    /// would not apprear in this operation. Use .locations to obtain the full alignment.
    pub operations: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy)]
pub enum AlignMode {
    Global,
    Prefix,
    Infix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignTask {
    Distance,
    Location,
    Alignment,
}

pub fn edlib_align(query: &[u8], target: &[u8], mode: AlignMode, task: AlignTask) -> Alignment {
    let mode_ed = match mode {
        AlignMode::Global => EdlibAlignMode_EDLIB_MODE_NW,
        AlignMode::Prefix => EdlibAlignMode_EDLIB_MODE_SHW,
        AlignMode::Infix => EdlibAlignMode_EDLIB_MODE_HW,
    };
    let task_ed = match task {
        AlignTask::Distance => EdlibAlignTask_EDLIB_TASK_DISTANCE,
        AlignTask::Location => EdlibAlignTask_EDLIB_TASK_LOC,
        AlignTask::Alignment => EdlibAlignTask_EDLIB_TASK_PATH,
    };
    unsafe {
        let config = edlibNewAlignConfig(-1, mode_ed, task_ed, std::ptr::null_mut(), 0);
        let align = edlibAlign(
            query.as_ptr() as *const i8,
            query.len() as i32,
            target.as_ptr() as *const i8,
            target.len() as i32,
            config,
        );
        if align.status != 0 {
            panic!("Edlib paniced!");
        } else {
            let dist = align.editDistance as u32;
            let locations: Option<Vec<(usize, usize)>> = (task != AlignTask::Distance).then(|| {
                let length = align.numLocations as usize;
                let starts = std::slice::from_raw_parts(align.startLocations, length).to_vec();
                let ends = std::slice::from_raw_parts(align.endLocations, length).to_vec();
                starts
                    .iter()
                    .zip(ends)
                    .map(|(&x, y)| (x as usize, y as usize))
                    .collect()
            });
            let operations = (task == AlignTask::Alignment).then(|| {
                let length = align.alignmentLength as usize;
                std::slice::from_raw_parts(align.alignment, length).to_vec()
            });
            edlibFreeAlignResult(align);
            Alignment {
                dist,
                locations,
                operations,
            }
        }
    }
}

pub fn global_dist(target: &[u8], query: &[u8]) -> u32 {
    global_dist_k(target, query, -1)
}

pub fn global_dist_k(target: &[u8], query: &[u8], k: i32) -> u32 {
    unsafe {
        let config = edlibNewAlignConfig(
            k,
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

pub fn local(target: &[u8], query: &[u8]) -> Vec<u8> {
    unsafe {
        let config = edlibNewAlignConfig(
            -1,
            EdlibAlignMode_EDLIB_MODE_HW,
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

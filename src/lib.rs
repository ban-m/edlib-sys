#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub mod bindings;

#[derive(Debug, Clone)]
pub struct Alignment {
    align: bindings::EdlibAlignResult,
    task: AlignTask,
    mode: AlignMode,
}

impl Alignment {
    pub fn dist(&self) -> u32 {
        self.align.editDistance as u32
    }
    /// Note that the end coordinate is *1-index*!
    pub fn location(&self) -> Option<(usize, usize)> {
        (self.task != AlignTask::Distance).then(|| unsafe {
            let length = self.align.numLocations as usize;
            let starts = std::slice::from_raw_parts(self.align.startLocations, length);
            let ends = std::slice::from_raw_parts(self.align.endLocations, length);
            (starts[0] as usize, ends[0] as usize)
        })
    }
    pub fn operations(&self) -> Option<&[u8]> {
        (self.task == AlignTask::Alignment).then(|| unsafe {
            let length = self.align.alignmentLength as usize;
            std::slice::from_raw_parts(self.align.alignment, length)
        })
    }
}

impl Drop for Alignment {
    fn drop(&mut self) {
        unsafe { bindings::edlibFreeAlignResult(self.align) };
    }
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

pub fn align(query: &[u8], target: &[u8], mode: AlignMode, task: AlignTask) -> Alignment {
    let mode_ed = match mode {
        AlignMode::Global => bindings::EdlibAlignMode_EDLIB_MODE_NW,
        AlignMode::Prefix => bindings::EdlibAlignMode_EDLIB_MODE_SHW,
        AlignMode::Infix => bindings::EdlibAlignMode_EDLIB_MODE_HW,
    };
    let task_ed = match task {
        AlignTask::Distance => bindings::EdlibAlignTask_EDLIB_TASK_DISTANCE,
        AlignTask::Location => bindings::EdlibAlignTask_EDLIB_TASK_LOC,
        AlignTask::Alignment => bindings::EdlibAlignTask_EDLIB_TASK_PATH,
    };
    unsafe {
        let config = bindings::edlibNewAlignConfig(-1, mode_ed, task_ed, std::ptr::null_mut(), 0);
        let align = bindings::edlibAlign(
            query.as_ptr() as *const i8,
            query.len() as i32,
            target.as_ptr() as *const i8,
            target.len() as i32,
            config,
        );
        if align.status != 0 {
            panic!("Edlib paniced!");
        } else {
            Alignment { align, task, mode }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{align, AlignMode, AlignTask};
    #[test]
    fn edlib_test() {
        let target = b"AAACCCTTT";
        let query = b"CCC";
        let task = AlignTask::Alignment;
        let mode = AlignMode::Infix;
        let aln = align(query, target, mode, task);
        assert_eq!(aln.dist(), 0);
        assert_eq!(aln.operations(), Some([0, 0, 0].as_slice()));
    }
}

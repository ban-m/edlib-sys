#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
//! # Edlib-sys
//!
//! This library is a tiny binding for the [edlib]() library.
//!  
//! ## Background
//!
//! In the DNA sequence analysis, one of the most fundamental problem is the comparison
//! between two DNA sequence, such as AACGTT versus AATGT.
//! To this end, let's use `|` and `X` character to express that two characters in these
//! two sequence do or do not correspond to each other, and use `-` character to
//! indicate that there is no corresponding character to one side of the string.
//! Then, one might argue that these
//! two strings *align* as follows:
//!
//! ```text
//! AACGTT
//! ||X||
//! AATGT-
//! ```
//!/
//! The intuitive argument behind this *alignment* is that, by *aligning* these two
//! strings in this way, we minimize the number of `X` and `-` characters -- which
//! intuitively measure how "different" the two strings.
//!
//! Edit distance (or more refferred to as "Levenshtein distance" by Math people) is
//! a rigorous formalization of this intuitive comparison of two strings;
//! It computes the minimum number of the 'X' and '-' characters to align two given strings,
//! and what is more, the *alignment* itself.
//!
//! There are a lot of implementation of the edit distance, and it is indeed one of the algorithm
//! that bioinformaticians learn at the age of seven or eight, whatever education they take.
//! But the problem is that, although an implementation is easy, the **fast** implementetion of the
//! edit distance is very hard, and there are still improvements of the algorithm made by assuming
//! several reasonable assumption on the DNA sequences.
//!
//! The edlib library is one of the fastest implmentation of the edit distance computation and
//! used by extensively here and there in the bioinformatics software.
//!
//! ## What does this library provide
//!
//! This crate provides a single API that call edlib's edit distance computation.
//!
//! ## Example
//! ```rust
//! use edlib_sys::{align, AlignTask, AlignMode};
//! let target = b"AAACCCTTT";
//! let query = b"CCC";
//! let task = AlignTask::Alignment;
//! let mode = AlignMode::Infix;
//! let aln = align(query, target, mode, task);
//! assert_eq!(aln.dist(), 0);
//! assert_eq!(aln.operations(), Some([0, 0, 0].as_slice()));
//! ```
//!
//!

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
mod bindings;

/// A struct for the result of an alignment computed by `edlib`.
/// All the members are not accessible from the callers, so use methods such as [`Alignment::location()`] or [`Alignment::operations()`] to
/// obtain the alignment information.
/// # Example
/// ```
/// use edlib_sys::{align, AlignTask, AlignMode};
/// let target = b"AAACCCTTT";
/// let query = b"CCC";
/// let task = AlignTask::Alignment;
/// let mode = AlignMode::Infix;
/// let aln = align(query, target, mode, task);
/// assert_eq!(aln.dist(), 0);
/// assert_eq!(aln.operations(), Some([0, 0, 0].as_slice()));
/// ```
#[derive(Debug, Clone)]
pub struct Alignment {
    align: bindings::EdlibAlignResult,
    task: AlignTask,
    mode: AlignMode,
}

impl Alignment {
    /// Return the edit distance.
    pub fn dist(&self) -> u32 {
        self.align.editDistance as u32
    }
    /// Return the range of the `target` sequence that aligns to the `query`.
    /// Note that the end coordinate is *1-index*!
    /// In other words, for a given tuple of `Some((start, end))` returned by the method call,
    /// one needs to slice the original (reference) string `target` as `target[start..end+1]` to
    /// obtain the sequence that matches to the query string.
    /// Use `self.locations()` to obtain all the locations.
    /// Return None if the alignment task is [`AlignTask::Distance`].
    /// # Example
    pub fn location(&self) -> Option<(usize, usize)> {
        (self.task != AlignTask::Distance).then(|| unsafe {
            let length = self.align.numLocations as usize;
            let starts = std::slice::from_raw_parts(self.align.startLocations, length);
            let ends = std::slice::from_raw_parts(self.align.endLocations, length);
            (starts[0] as usize, ends[0] as usize)
        })
    }
    /// Return all the alignment location of the `target` that aligns to the `query`.
    /// Return None if the alignment task is `[AlignTask::Distance]`.
    /// To supress the memory allocation, this function returns the slice of `i32` -- it
    /// is the responsibility of the callers to convert the `i32` into `usize`.
    pub fn locations(&self) -> Option<(&[i32], &[i32])> {
        (self.task != AlignTask::Distance).then(|| unsafe {
            let length = self.align.numLocations as usize;
            let starts = std::slice::from_raw_parts(self.align.startLocations, length);
            let ends = std::slice::from_raw_parts(self.align.endLocations, length);
            (starts, ends)
        })
    }
    /// Return the alignment operation of the alignment between
    /// `target` sequence and `query` sequence.
    /// Note that the operations is the alignment between the aligned region of the
    /// target sequence and the entire query sequence.
    /// In other words, if one use [`AlignMode`] other than [`AlignMode::Global`],
    /// one needs to call [`Alignment::location()`] to get the aligned region of the
    /// `target` sequence.
    ///
    /// The alignment operations are defined as follows:
    ///
    /// - 0: Base match
    /// - 1: Insertion to the target
    /// - 2: Deletion from the target
    /// - 3: Base mismatch
    ///
    /// Note also that this function returns one of the otpimal operation --
    /// in fact, enumerating all the optimal alignment is too hard to compute (suppose very long Poly-A sequences).
    /// # Example
    /// ```
    /// use edlib_sys::{align, AlignTask, AlignMode};
    /// // Infix mode example
    /// let target = b"ACCGCCT";
    /// let query =  b"CCCCC";
    /// let task = AlignTask::Alignment;
    /// let mode = AlignMode::Infix;
    /// let aln = align(query, target, mode, task);
    /// assert_eq!(aln.dist(), 1);
    /// assert_eq!(aln.location(), Some((1, 5)));
    /// assert_eq!(aln.operations(), Some([0, 0, 3, 0, 0].as_slice()));
    ///
    /// // Prefix mode example
    /// // The alignment is
    /// // ```text
    /// // ACGTC-G
    /// //  |||| |
    /// // -CGTCCG
    /// // ```
    /// let target = b"ACGTCGT";
    /// let query =  b"CGTCCG";
    /// let task = AlignTask::Alignment;
    /// let mode = AlignMode::Prefix;
    /// let aln = align(query, target, mode, task);
    /// assert_eq!(aln.dist(), 2);
    /// assert_eq!(aln.location(), Some((0, 5)));
    /// assert_eq!(aln.operations(), Some([2, 0, 0, 0, 0, 1, 0].as_slice()));
    /// ```
    pub fn operations(&self) -> Option<&[u8]> {
        (self.task == AlignTask::Alignment).then(|| unsafe {
            let length = self.align.alignmentLength as usize;
            std::slice::from_raw_parts(self.align.alignment, length)
        })
    }
}

impl Drop for Alignment {
    // Safe -- because the memory of the edlib is allocated by edlib,
    // so let it go to the edlib's deallocator.
    fn drop(&mut self) {
        unsafe { bindings::edlibFreeAlignResult(self.align) };
    }
}

/// The alignment mode to be used.
/// - Global: Align entire sequence between `target` and `query`. Corresponds to `EdlibAlignMode_EDLIB_MODE_NW`.
/// - Prefix: Align the prefix of the `target` and `query`. Corresponds to `EdlibAlignMode_EDLIB_MODE_SHW`
/// - Infix: Align the `query` into the substring of the `target`. Corresnponds to `EdlibAlignMode_EDLIB_MODE_HW`
#[derive(Debug, Clone, Copy)]
pub enum AlignMode {
    Global,
    Prefix,
    Infix,
}

/// The alignment task to be used.
/// - Distance: Only the distance would be computed.
/// - Location: Distance & the location on the `target` squence would be computed.
/// - Alignment: Distance, the location, and the alignment operations would be computed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignTask {
    Distance,
    Location,
    Alignment,
}

/// Align the `query` to the `target` sequence.
/// # Example
/// ```
/// use edlib_sys::{align, AlignTask, AlignMode};
/// // Infix mode example
/// let target = b"ACCGCCT";
/// let query =  b"CCCCC";
/// let task = AlignTask::Alignment;
/// let mode = AlignMode::Infix;
/// let aln = align(query, target, mode, task);
/// assert_eq!(aln.dist(), 1);
/// assert_eq!(aln.location(), Some((1, 5)));
/// assert_eq!(aln.operations(), Some([0, 0, 3, 0, 0].as_slice()));
///
/// // Prefix mode example
/// // The alignment is
/// // ```text
/// // ACGTC-G
/// //  |||| |
/// // -CGTCCG
/// // ```
/// let target = b"ACGTCGT";
/// let query =  b"CGTCCG";
/// let task = AlignTask::Alignment;
/// let mode = AlignMode::Prefix;
/// let aln = align(query, target, mode, task);
/// assert_eq!(aln.dist(), 2);
/// assert_eq!(aln.location(), Some((0, 5)));
/// assert_eq!(aln.operations(), Some([2, 0, 0, 0, 0, 1, 0].as_slice()));
/// ```
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

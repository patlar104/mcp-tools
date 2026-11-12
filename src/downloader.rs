//! Shared data containers; validation and download execution are not implemented.

use std::path::PathBuf;

/// A requested source and local destination, without URL or path validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadRequest {
    /// Source URL to validate when download execution is implemented.
    pub url: String,
    /// Destination file path.
    pub destination: PathBuf,
}

/// A byte-range segment using start-inclusive, end-exclusive `[start, end)` bounds.
///
/// Valid segments require `start < end`. This plain data type does not enforce
/// that invariant; future scheduling and request validation must check it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRangeSegment {
    /// Inclusive byte offset of the first byte.
    pub start: u64,
    /// Exclusive byte offset immediately after the last byte.
    pub end: u64,
}

/// A progress snapshot for one download, without completion or consistency checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DownloadProgress {
    /// Unique bytes written to the destination, excluding duplicate retry work.
    pub downloaded_bytes: u64,
    /// Expected file size, or `None` when unknown.
    pub total_bytes: Option<u64>,
}

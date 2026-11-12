//! Shared foundation for a parallel downloader with a future thin MCP layer.

pub mod downloader;

pub use downloader::{ByteRangeSegment, DownloadProgress, DownloadRequest};

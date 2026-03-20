//! File system scanner.

use std::fs;
use std::path::{Path, PathBuf};

/// Events generated during a filesystem scan.
#[derive(Debug, PartialEq)]
pub enum ScanEvent {
        /// Entered a directory with the given name.
        EnterDir(String),
        /// Found a file with the given name and size in bytes.
        File(String, u64),
        /// Finished scanning current directory.
        ExitDir,
}

/// A lazy iterator that scans the filesystem and yields [`ScanEvent`]s.
pub struct Scanner {
        /// Stack of directory iterators for recursion.
        stack: Vec<(String, fs::ReadDir)>,
        /// Indicating whether we just started scanning.
        is_first: bool,
        /// Root path to start from.
        root_path: PathBuf,
}

impl Scanner {
        /// Create a new scanner at the given path.
        pub fn new<P: AsRef<Path>>(path: P) -> Self {
                Self {
                        stack: Vec::new(),
                        is_first: true,
                        root_path: path.as_ref().to_path_buf(),
                }
        }
}

impl Iterator for Scanner {
        type Item = ScanEvent;

        fn next(&mut self) -> Option<Self::Item> {
                // Handle root entry first.
                if self.is_first {
                        self.is_first = false;
                        let root_name = self
                                .root_path
                                .file_name()
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_else(|| "/".to_string());

                        if let Ok(rd) = fs::read_dir(&self.root_path) {
                                self.stack.push((root_name.clone(), rd));
                                return Some(ScanEvent::EnterDir(root_name));
                        } else {
                                return None;
                        }
                }

                while let Some((_, iter)) = self.stack.last_mut() {
                        if let Some(entry) = iter.next() {
                                let entry = match entry {
                                        Ok(e) => e,
                                        Err(_) => continue,
                                };

                                // Get filename.
                                let name = entry.file_name().to_string_lossy().to_string();

                                if entry.file_type().unwrap().is_dir() {
                                        // Add this directory to stack so we can handle it later.
                                        if let Ok(rd) = fs::read_dir(entry.path()) {
                                                self.stack.push((name.clone(), rd));
                                                return Some(ScanEvent::EnterDir(name));
                                        }
                                } else {
                                        return Some(ScanEvent::File(
                                                name,
                                                entry.metadata().unwrap().len(),
                                        ));
                                }
                        } else {
                                // Done with current directory.
                                self.stack.pop();
                                return Some(ScanEvent::ExitDir);
                        }
                }

                None
        }
}

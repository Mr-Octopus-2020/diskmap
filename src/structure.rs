//! Core file system structure.

use crate::scanner::{ScanEvent, Scanner};
use std::path::Path;

/// Arena tree snapshot of the file system.
#[derive(Debug)]
pub struct DiskMap {
        /// Ordered list of nodes in the filesystem.
        pub nodes: Vec<Node>,
        /// Index of the root node.
        /// The map is empty if it doesn't have a root.
        pub root: Option<u32>,
}

/// Represents a file or a directory.
#[derive(Debug)]
pub struct Node {
        pub name: String,
        pub size: u64,
        pub kind: NodeKind,
        pub parent: Option<u32>,
        pub first_child: Option<u32>,
        pub next_sibling: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NodeKind {
        File,
        Dir,
}

impl DiskMap {
        /// Create a new empty DiskMap.
        pub fn new() -> Self {
                Self {
                        nodes: Vec::new(),
                        root: None,
                }
        }

        /// Add a node and return its index.
        pub fn add_node(&mut self, node: Node) -> u32 {
                let idx = self.nodes.len() as u32;

                if let Some(parent_idx) = node.parent {
                        // Update internal link.
                        self.link_child(parent_idx, idx);
                } else if self.root.is_none() {
                        self.root = Some(idx);
                }

                self.nodes.push(node);
                idx
        }

        /// Link child to parent.
        fn link_child(&mut self, parent_idx: u32, child_idx: u32) {
                let parent = &mut self.nodes[parent_idx as usize];

                // Find the last child.
                if let Some(mut current_child) = parent.first_child {
                        loop {
                                let current = &self.nodes[current_child as usize];
                                if let Some(next) = current.next_sibling {
                                        current_child = next;
                                } else {
                                        break;
                                }
                        }
                        self.nodes[current_child as usize].next_sibling = Some(child_idx);
                } else {
                        parent.first_child = Some(child_idx);
                }
        }

        /// Display tree with indentation.
        pub fn display(&self) {
                if let Some(root_idx) = self.root {
                        self.display_recursive(root_idx, 0);
                }
        }

        fn display_recursive(&self, idx: u32, depth: usize) {
                let node = &self.nodes[idx as usize];
                let prefix = if node.kind == NodeKind::Dir {
                        "[DIR]"
                } else {
                        "[FILE]"
                };
                println!(
                        "{}{} {} ({} bytes)",
                        "  ".repeat(depth),
                        prefix,
                        node.name,
                        node.size
                );

                let mut next = node.first_child;
                while let Some(c_idx) = next {
                        self.display_recursive(c_idx, depth + 1);
                        next = self.nodes[c_idx as usize].next_sibling;
                }
        }

        /// Build a DiskMap from a path.
        pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
                let mut map = Self::new();
                let scanner = Scanner::new(path);
                let mut stack = Vec::new();

                for event in scanner {
                        match event {
                                ScanEvent::EnterDir(name) => {
                                        let parent = stack.last().copied();
                                        let idx = map.add_node(Node::new(
                                                name,
                                                NodeKind::Dir,
                                                0,
                                                parent,
                                        ));
                                        stack.push(idx);
                                }
                                ScanEvent::File(name, size) => {
                                        let parent = stack.last().copied();
                                        map.add_node(Node::new(name, NodeKind::File, size, parent));
                                        if let Some(&p_idx) = stack.last() {
                                                map.nodes[p_idx as usize].size += size;
                                        }
                                }
                                ScanEvent::ExitDir => {
                                        if let Some(finished_idx) = stack.pop() {
                                                let finished_size =
                                                        map.nodes[finished_idx as usize].size;
                                                if let Some(&p_idx) = stack.last() {
                                                        map.nodes[p_idx as usize].size +=
                                                                finished_size;
                                                }
                                        }
                                }
                        }
                }
                map
        }
}

impl From<&Path> for DiskMap {
        fn from(path: &Path) -> Self {
                DiskMap::from_path(path)
        }
}

impl Node {
        pub fn new(name: String, kind: NodeKind, size: u64, parent: Option<u32>) -> Self {
                Self {
                        name,
                        size,
                        kind,
                        parent,
                        first_child: None,
                        next_sibling: None,
                }
        }
}

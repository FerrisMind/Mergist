use std::collections::BTreeMap;

use crate::models::FileEntry;

#[derive(Debug)]
enum Node {
    Dir(BTreeMap<String, Node>),
    File,
}

impl Node {
    fn ensure_dir(&mut self, name: &str) -> &mut Node {
        match self {
            Node::Dir(children) => children
                .entry(name.to_string())
                .or_insert_with(|| Node::Dir(BTreeMap::new())),
            Node::File => panic!("cannot add dir to file node"),
        }
    }
}

pub fn generate_directory_tree(files: &[&FileEntry], subdirectory: Option<&str>) -> String {
    let mut root = Node::Dir(BTreeMap::new());
    let prefix = subdirectory.unwrap_or("").trim_end_matches('/');

    for file in files {
        let mut path = file.path.as_str();
        if !prefix.is_empty() && path.starts_with(prefix) {
            path = path.trim_start_matches(prefix).trim_start_matches('/');
        }
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
        let mut current = &mut root;
        for (idx, part) in parts.iter().enumerate() {
            let is_last = idx == parts.len() - 1;
            if is_last {
                match current {
                    Node::Dir(children) => {
                        children.entry(part.to_string()).or_insert(Node::File);
                    }
                    Node::File => {}
                }
            } else {
                current = current.ensure_dir(part);
            }
        }
    }

    fn fmt(node: &Node, prefix: &str, is_last: bool, acc: &mut String) {
        let connector = if prefix.is_empty() {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };
        match node {
            Node::Dir(children) => {
                // skip printing root
                if !prefix.is_empty() {
                    acc.push_str(prefix);
                    acc.push_str(connector);
                    acc.push('\n');
                }
                let mut iter = children.iter().peekable();
                while let Some((name, child)) = iter.next() {
                    let child_last = iter.peek().is_none();
                    let new_prefix = if prefix.is_empty() {
                        String::new()
                    } else if is_last {
                        format!("{prefix}    ")
                    } else {
                        format!("{prefix}│   ")
                    };
                    let display_line = format!(
                        "{new_prefix}{}{}\n",
                        if child_last {
                            "└── "
                        } else {
                            "├── "
                        },
                        name
                    );
                    acc.push_str(&display_line);
                    if let Node::Dir(_) = child {
                        fmt(
                            child,
                            &format!("{new_prefix}{}", if child_last { "    " } else { "│   " }),
                            true,
                            acc,
                        );
                    }
                }
            }
            Node::File => {}
        }
    }

    let mut entries: Vec<(&String, &Node)> = match &root {
        Node::Dir(children) => children.iter().collect(),
        Node::File => Vec::new(),
    };

    // Ensure directories first, then files, case-insensitive lexicographically
    entries.sort_by(
        |(a_name, a_node), (b_name, b_node)| match (a_node, b_node) {
            (Node::Dir(_), Node::File) => std::cmp::Ordering::Less,
            (Node::File, Node::Dir(_)) => std::cmp::Ordering::Greater,
            _ => a_name.to_lowercase().cmp(&b_name.to_lowercase()),
        },
    );

    let mut acc = String::new();
    for (idx, (name, node)) in entries.iter().enumerate() {
        let is_last = idx == entries.len() - 1;
        acc.push_str(if is_last { "└── " } else { "├── " });
        acc.push_str(name);
        acc.push('\n');
        if let Node::Dir(_) = node {
            fmt(node, "", is_last, &mut acc);
        }
    }

    acc
}

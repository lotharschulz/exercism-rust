use std::fmt::Debug;

#[derive(Debug)]
pub struct Tree<T: Debug + Ord> {
    label: T,
    children: Vec<Tree<T>>,
}

impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Tree {
            label,
            children: Vec::new(),
        }
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    /// Find a node with the given label and return the path to it (as indices)
    fn find_path_indices(&self, target: &T) -> Option<Vec<usize>> {
        if &self.label == target {
            return Some(Vec::new());
        }

        for (i, child) in self.children.iter().enumerate() {
            if let Some(mut path) = child.find_path_indices(target) {
                path.insert(0, i);
                return Some(path);
            }
        }

        None
    }

    /// Find a node with the given label and return the path to it
    fn find_path(&self, target: &T) -> Option<Vec<&T>> {
        if &self.label == target {
            return Some(vec![&self.label]);
        }

        for child in &self.children {
            if let Some(mut path) = child.find_path(target) {
                path.insert(0, &self.label);
                return Some(path);
            }
        }

        None
    }

    /// Reparent the tree from the perspective of the given node
    pub fn pov_from(&mut self, from: &T) -> bool {
        if &self.label == from {
            return true;
        }

        // Find the path to the target node
        let path_indices = match self.find_path_indices(from) {
            Some(p) => p,
            None => return false,
        };

        // Reverse the tree by following the path
        self.reparent_along_path(&path_indices);
        true
    }

    /// Recursively reparent along the given path
    fn reparent_along_path(&mut self, path: &[usize]) {
        if path.is_empty() {
            return;
        }

        let child_idx = path[0];
        let remaining_path = &path[1..];

        // Extract the child on the path
        let mut child = self.children.remove(child_idx);

        // Recursively reparent within the child
        child.reparent_along_path(remaining_path);

        // Build the old self tree (using child's label temporarily via replace, will be swapped)
        let old_self = Tree {
            label: std::mem::replace(&mut self.label, child.label),
            children: std::mem::take(&mut self.children),
        };

        let mut child_children_temp = std::mem::take(&mut child.children);

        // Determine where to attach old_self
        if remaining_path.is_empty() {
            // Child is the target, attach directly to it
            child_children_temp.push(old_self);
        } else {
            // After reparenting, child has:
            // - The target's original children (if any)
            // - The reparenting chain (added as last child)
            if child_children_temp.len() == 1 {
                // Only the reparenting chain, find the deepest point
                let mut current = &mut child_children_temp[0];
                while !current.children.is_empty() {
                    let last_idx = current.children.len() - 1;
                    current = &mut current.children[last_idx];
                }
                current.children.push(old_self);
            } else {
                // Target has original children, attach to the last child (reparenting chain)
                let last_idx = child_children_temp.len() - 1;
                child_children_temp[last_idx].children.push(old_self);
            }
        }

        // Self now has child's label (from replace above) and the modified children
        self.children = child_children_temp;
    }

    /// Find the path between two nodes
    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        // First, reparent from the 'from' node
        if !self.pov_from(from) {
            return None;
        }

        // Now find the path to 'to' (which is just a simple traversal)
        self.find_path(to)
    }
}

// Equality traits
/// Custom equality that ignores the order of children
impl<T: Debug + Ord> PartialEq for Tree<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.label != other.label {
            return false;
        }

        if self.children.len() != other.children.len() {
            return false;
        }

        // Sort children for comparison
        let mut self_children: Vec<_> = self.children.iter().collect();
        let mut other_children: Vec<_> = other.children.iter().collect();

        self_children.sort();
        other_children.sort();

        self_children == other_children
    }
}

impl<T: Debug + Ord> Eq for Tree<T> {}

// Ordering traits (these require Eq)
/// Ordering for trees (needed for sorting children)
impl<T: Debug + Ord> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Debug + Ord> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.label.cmp(&other.label) {
            std::cmp::Ordering::Equal => {
                let mut self_children: Vec<_> = self.children.iter().collect();
                let mut other_children: Vec<_> = other.children.iter().collect();

                self_children.sort();
                other_children.sort();

                self_children.cmp(&other_children)
            }
            other => other,
        }
    }
}

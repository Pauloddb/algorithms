pub struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T, left: Option<Box<TreeNode<T>>>, right: Option<Box<TreeNode<T>>>) -> Self {
        Self { val, left, right }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn bfs_test() {
        let root = Some(Box::new(TreeNode::new(
            "A",
            Some(Box::new(TreeNode::new(
                "B",
                Some(Box::new(TreeNode::new("D", None, None))),
                Some(Box::new(TreeNode::new("E", None, None))),
            ))),
            Some(Box::new(TreeNode::new(
                "C",
                Some(Box::new(TreeNode::new("F", None, None))),
                Some(Box::new(TreeNode::new("G", None, None))),
            ))),
        )));

        let mut queue = VecDeque::new();

        if let Some(r) = &root {
            queue.push_back(r);
        }

        let mut found = None;
        let target = "F";

        while let Some(node) = queue.pop_front() {
            if node.val == target {
                found = Some(target);
                break;
            }

            if let Some(left) = &node.left {
                queue.push_back(left);
            }
            if let Some(right) = &node.right {
                queue.push_back(right);
            }
        }

        assert_eq!(found, Some("F"));
    }
}

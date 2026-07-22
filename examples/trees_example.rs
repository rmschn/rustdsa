// Trees (General Tree)
// A hierarchical data structure with a root node and children.
// Each node has data and a list of child nodes.
// Traversals: depth-first (preorder, inorder, postorder), breadth-first.

use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T> {
    data: T,
    children: Vec<TreeNode<T>>,
}

impl<T: Debug> TreeNode<T> {
    fn new(data: T) -> Self {
        TreeNode {
            data,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: TreeNode<T>) {
        self.children.push(child);
    }

    // Preorder traversal (root, then children)
    fn preorder(&self, indent: usize) {
        println!("{:indent$}{:?}", "", self.data, indent = indent);
        for child in &self.children {
            child.preorder(indent + 2);
        }
    }

    // Depth (height) of tree
    fn depth(&self) -> usize {
        1 + self
            .children
            .iter()
            .map(|c| c.depth())
            .max()
            .unwrap_or(0)
    }

    // Count nodes
    fn count(&self) -> usize {
        1 + self.children.iter().map(|c| c.count()).sum::<usize>()
    }
}

fn main() {
    // Build tree: root -> [child1, child2] -> [grandchild]
    let mut root = TreeNode::new("root");

    let mut child1 = TreeNode::new("child1");
    child1.add_child(TreeNode::new("grandchild1"));
    child1.add_child(TreeNode::new("grandchild2"));

    let mut child2 = TreeNode::new("child2");
    child2.add_child(TreeNode::new("grandchild3"));

    root.add_child(child1);
    root.add_child(child2);

    println!("Tree preorder traversal:");
    root.preorder(0);

    println!("Tree depth: {}", root.depth());
    println!("Node count: {}", root.count());

    // File system representation
    let mut fs_root = TreeNode::new("/");
    let mut home = TreeNode::new("home");
    let mut user = TreeNode::new("user");
    user.add_child(TreeNode::new("documents"));
    user.add_child(TreeNode::new("downloads"));
    home.add_child(user);
    fs_root.add_child(home);
    fs_root.add_child(TreeNode::new("tmp"));

    println!("\nFile system tree:");
    fs_root.preorder(0);
}

use rand::Rng;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct TreeNode {
    id: u32,
    // TODO: Is there a better way to do this?
    // In the book (page 343) it's done like that,
    // but the consequence is that I can just mutate
    // any immutable TreeNode
    children: RefCell<Vec<Rc<TreeNode>>>,
    parent: RefCell<Weak<TreeNode>>,
}

impl TreeNode {
    fn new(id: u32) -> TreeNode {
        TreeNode {
            id,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        }
    }

    fn depth(&self) -> u32 {
        match self.parent.borrow().upgrade() {
            None => 0,
            Some(parent) => parent.depth() + 1,
        }
    }

    fn size(&self) -> u32 {
        1 + self.children.borrow().iter().map(|c| c.size()).sum::<u32>()
    }

    // TODO: Could this be a method? If it was, I couldn't return a pointer
    fn find_root(node: &Rc<TreeNode>) -> Rc<TreeNode> {
        match node.parent.borrow().upgrade() {
            None => return Rc::clone(node),
            Some(parent_rc) => return Self::find_root(&parent_rc),
        };
    }

    // TODO: Could this be a method? If it was, how would I get the parent pointer?
    fn add_child(parent: &Rc<TreeNode>, child: &Rc<TreeNode>) {
        parent.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Rc::downgrade(parent);
    }

    fn representation_string(&self) -> String {
        let mut result = String::new();
        result += &("  ".repeat(self.depth() as usize).to_string()
            + &format!(
                "{} (Size: {}, Depth: {})",
                self.id,
                self.size(),
                self.depth()
            )
            + "\n");
        for child in self.children.borrow().iter() {
            result += &child.representation_string();
        }
        result
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.representation_string())
    }
}

impl Drop for TreeNode {
    fn drop(&mut self) {
        println!("Dropping {}", self.id)
    }
}

pub fn main() {
    let mut nodes = vec![];
    let mut rng = rand::thread_rng();

    for i in 0..20 {
        let node = Rc::new(TreeNode::new(i));

        if i != 0 {
            let parent_id: usize = rng.gen_range(0, i as usize);
            TreeNode::add_child(&nodes[parent_id], &node);
        }

        nodes.push(node);
    }
    println!("{}", nodes[0]);

    for node in nodes.iter() {
        assert_eq!(TreeNode::find_root(node).id, 0);
    }

    println!(
        "Max depth: {}",
        nodes.iter().map(|n| n.depth()).max().unwrap()
    );

    let subtree = Rc::clone(&nodes[1]);
    println!(
        "Dropping all nodes, except subtree 1 of size {}",
        subtree.size()
    );
    nodes.clear();
    println!("{}", subtree);
}

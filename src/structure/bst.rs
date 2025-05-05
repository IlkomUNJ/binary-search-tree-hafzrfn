use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    #[allow(dead_code)] 
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value (Iterative version)
    pub fn tree_search(mut current_node_link: BstNodeLink, value: &i32) -> Option<BstNodeLink> {
        loop {
            let current_node = current_node_link.borrow();
            match current_node.key {
                Some(key) => {
                    if *value == key {
                        return Some(Rc::clone(&current_node_link));
                    }
                    let next_node = if *value < key {
                        current_node.left.clone()
                    } else {
                        current_node.right.clone()
                    };
                    drop(current_node); // Explicitly drop the borrow
                    if let Some(next) = next_node {
                        current_node_link = next;
                    } else {
                        return None; // Value not found
                    }
                }
                None => return None, // Should not happen in a valid tree with Option<i32> keys
            }
        }
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */

    // Revised minimum function that takes BstNodeLink
    pub fn minimum_nodelink(mut node: BstNodeLink) -> BstNodeLink { // Removed mut
        loop {
            let left_child = node.borrow().left.clone();
            if let Some(left_node) = left_child {
                 node = left_node;
            } else {
                 return node;
            }
        }
    }

    // Keeping the original recursive version but it's less idiomatic with Rc/RefCell for traversal roots
    // Revised maximum function that takes BstNodeLink
    pub fn maximum_nodelink(mut node: BstNodeLink) -> BstNodeLink { // Removed mut
        loop {
            let right_child = node.borrow().right.clone();
            if let Some(right_node) = right_child {
                 node = right_node;
            } else {
                 return node;
            }
        }
    }


    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        BstNode::get_root(&parent.unwrap())
    }

    /**
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // case 1: node has a right child
        if let Some(right_node) = &x_node.borrow().right {
            return Some(BstNode::minimum_nodelink(Rc::clone(right_node)));
        }

        // case 2: node has no right child
        let mut current_node = Rc::clone(x_node);
        let mut parent_node = BstNode::upgrade_weak_to_strong(current_node.borrow().parent.clone());

        while let Some(p_node) = parent_node {
            if let Some(p_left) = &p_node.borrow().left {
                if Rc::ptr_eq(&current_node, p_left) {
                    return Some(p_node.clone()); // current_node is a left child, return a clone of parent
                }
            }
            // If current_node is the right child or parent has no left child (and we are right), move up
            current_node = p_node.clone();
            parent_node = BstNode::upgrade_weak_to_strong(current_node.borrow().parent.clone());
        }

        None // current_node is the maximum element
    }


    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)] // Keeping for reference, but `tree_successor` is preferred
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        let right_node = &x_node.borrow().right.clone();
        if right_node.is_some(){
            return Some(BstNode::minimum_nodelink(right_node.clone().unwrap()));
        }

        let mut current_node = Rc::clone(x_node);
        let mut parent_node = BstNode::upgrade_weak_to_strong(current_node.borrow().parent.clone());

        while let Some(p_node) = parent_node {
            // If current_node is the right child of its parent, continue moving up.
            if let Some(p_right) = &p_node.borrow().right {
                 if Rc::ptr_eq(&current_node, p_right) {
                     current_node = p_node.clone();
                     parent_node = BstNode::upgrade_weak_to_strong(current_node.borrow().parent.clone());
                 } else {
                     // current_node is the left child
                     return Some(p_node.clone()); // Return a clone
                 }
            } else if let Some(p_left) = &p_node.borrow().left {
                 if Rc::ptr_eq(&current_node, p_left) {
                     // current_node is the left child and parent has no right child
                      return Some(p_node.clone()); // Return a clone
                 } else {
                     // Parent has no right child, and current_node is not the left child.
                     current_node = p_node.clone();
                     parent_node = BstNode::upgrade_weak_to_strong(current_node.borrow().parent.clone());
                 }

            } else {
                 // Parent has no children. This shouldn't happen in a valid tree above leaves with parents.
                 current_node = p_node.clone();
                 parent_node = BstNode::upgrade_weak_to_strong(current_node.borrow().parent.clone());
            }
        }
        None
    }


    /**
     * Insert a new node with the given key into the BST rooted at `root`.
     * Returns the updated root of the tree.
     */
    pub fn tree_insert(root: Option<BstNodeLink>, z_key: i32) -> BstNodeLink {
        let z_node = BstNode::new_bst_nodelink(z_key);
        let mut y: Option<BstNodeLink> = None; // trailing pointer
        let mut x = root.clone(); // current node

        while let Some(current_x) = x.clone() {
            // Prevent inserting duplicate keys
            if current_x.borrow().key.unwrap() == z_key {
                println!("Key {} already exists, not inserting.", z_key);
                return root.unwrap(); // Return the original root
            }

            // If not a duplicate, update y and move to the next node
            y = Some(Rc::clone(&current_x));
            if z_key < current_x.borrow().key.unwrap() {
                x = current_x.borrow().left.clone();
            } else {
                x = current_x.borrow().right.clone();
            }
        }

        // y is the parent of z
        z_node.borrow_mut().parent = y.clone().map(|node| BstNode::downgrade(&node));

        if y.is_none() {
            // z is the root
            z_node
        } else if z_key < y.as_ref().unwrap().borrow().key.unwrap() {
            // z is the left child
            y.unwrap().borrow_mut().left = Some(z_node.clone()); // Use clone
            root.unwrap() // Root doesn't change if y exists
        } else {
            // z is the right child
            y.unwrap().borrow_mut().right = Some(z_node.clone()); // Use clone
             root.unwrap() // Root doesn't change if y exists
        }
    }


    /**
     */
    pub fn transplant(root: BstNodeLink, u: BstNodeLink, v: Option<BstNodeLink>) -> BstNodeLink { // Removed mut root
        let u_parent = BstNode::upgrade_weak_to_strong(u.borrow().parent.clone());

        if u_parent.is_none() {
            // u is the root
            if let Some(v_node) = v.clone() {
                v_node.borrow_mut().parent = None;
                v_node // The new root is v
            } else {
                 // If v is None and u was the root, the tree becomes empty.
                 // Returning a dummy node with a sentinel value (-1) to indicate empty.
                 BstNode::new_bst_nodelink(-1) // Placeholder for empty tree if root must be BstNodeLink
            }
        } else if let Some(u_p) = u_parent {
            // If 'u' has a parent ('u_p')
             let mut u_p_mut = u_p.borrow_mut(); // Mutably borrow the parent
            // Determine if 'u' is the left or right child of its parent
            if let Some(ref left_child) = u_p_mut.left {
                if Rc::ptr_eq(left_child, &u) {
                    u_p_mut.left = v.clone();
                } else {
                    u_p_mut.right = v.clone();
                }
            } else if let Some(ref right_child) = u_p_mut.right {
                 if Rc::ptr_eq(right_child, &u) {
                     u_p_mut.right = v.clone();
                 }
                 // If u has a parent, it MUST be either the left or right child.
                 // The case where parent has no children links to u
                 // should not happen in a consistent tree structure.
            }
            drop(u_p_mut); // Explicitly drop the mutable borrow of the parent

            if let Some(v_node) = v {
                v_node.borrow_mut().parent = Some(BstNode::downgrade(&u_p));
            }

            root // The root remains the same unless u was the root
        } else {
             // Should not reach here if u is in a valid tree
             root
        }
    }


    /**
     * Deletes the node `z` from the BST rooted at `root`.
     * Returns the new root of the tree.
     */
    pub fn tree_delete(root: BstNodeLink, z: BstNodeLink) -> BstNodeLink { // Removed mut root
        let z_borrowed = z.borrow();
        let z_left = z_borrowed.left.clone();
        let z_right = z_borrowed.right.clone();
        drop(z_borrowed); // Explicitly drop the borrow of z


        if z_left.is_none() {
            // Case 1: z has no left child
             BstNode::transplant(root, Rc::clone(&z), z_right)
        } else if z_right.is_none() {
            // Case 2: z has no right child
            BstNode::transplant(root, Rc::clone(&z), z_left)
        } else {
            // Case 3: z has two children
            let y = BstNode::minimum_nodelink(z_right.clone().unwrap()); // y is the successor

            // Get y's parent to check if y is z's direct right child
            let y_parent = BstNode::upgrade_weak_to_strong(y.borrow().parent.clone());

            // Check if y is not z's direct right child
            if !Rc::ptr_eq(&y_parent.as_ref().unwrap(), &z) {
                // Case 3a: y is not z's right child
                 let y_right = y.borrow().right.clone(); // Get y's right child
                 BstNode::transplant(root.clone(), Rc::clone(&y), y_right); // Added semicolon

                y.borrow_mut().right = z_right.clone();
                if let Some(z_r) = z_right {
                    // Update the parent pointer of z's original right child to y.
                    z_r.borrow_mut().parent = Some(BstNode::downgrade(&y));
                }

            } // End of Case 3a

            // Case 3b: y is z's right child (this is handled implicitly if Case 3a didn't apply,
            // as y's original right child would already be None if it was z's direct child).
            let new_root = BstNode::transplant(root, Rc::clone(&z), Some(Rc::clone(&y)));

            // Make y's left child z's original left child.
            // z must have a left child in Case 3.
            let z_left = z.borrow().left.clone().unwrap(); // Get z's left child
            y.borrow_mut().left = Some(z_left.clone()); // Set y's left child
            z_left.borrow_mut().parent = Some(BstNode::downgrade(&y)); // Update z.left's parent to y

            new_root // Return the potentially new root
        }
    }


    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    #[allow(dead_code)]
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    // These checks for children and key might not align with a standard
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                    || x.borrow().key.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink
    #[allow(dead_code)] // Not used in the core insert/delete logic
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    #[allow(dead_code)] // Not used in the core insert/delete logic
    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        anode.borrow().key == bnode.borrow().key
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        node.and_then(|weak_node| weak_node.upgrade())
    }
}
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
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

    #[allow(dead_code)]
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

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
                    drop(current_node);
                    if let Some(next) = next_node {
                        current_node_link = next;
                    } else {
                        return None;
                    }
                }
                None => return None,
            }
        }
    }

    pub fn minimum_nodelink(mut node: BstNodeLink) -> BstNodeLink {
        loop {
            let left_child = node.borrow().left.clone();
            if let Some(left_node) = left_child {
                 node = left_node;
            } else {
                 return node;
            }
        }
    }

    pub fn maximum_nodelink(mut node: BstNodeLink) -> BstNodeLink {
        loop {
            let right_child = node.borrow().right.clone();
            if let Some(right_node) = right_child {
                 node = right_node;
            } else {
                 return node;
            }
        }
    }

    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        BstNode::get_root(&parent.unwrap())
    }

    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        if let Some(right_node) = &x_node.borrow().right {
            return Some(BstNode::minimum_nodelink(Rc::clone(right_node)));
        }

        let mut current_node = Rc::clone(x_node);
        let mut parent_node = BstNode::upgrade_weak_to_strong(current_node.borrow().parent.clone());

        while let Some(p_node) = parent_node {
            if let Some(p_left) = &p_node.borrow().left {
                if Rc::ptr_eq(&current_node, p_left) {
                    return Some(p_node.clone());
                }
            }
            current_node = p_node.clone();
            parent_node = BstNode::upgrade_weak_to_strong(current_node.borrow().parent.clone());
        }

        None
    }

    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        let right_node = &x_node.borrow().right.clone();
        if right_node.is_some(){
            return Some(BstNode::minimum_nodelink(right_node.clone().unwrap()));
        }

        let mut current_node = Rc::clone(x_node);
        let mut parent_node = BstNode::upgrade_weak_to_strong(current_node.borrow().parent.clone());

        while let Some(p_node) = parent_node {
            if let Some(p_right) = &p_node.borrow().right {
                 if Rc::ptr_eq(&current_node, p_right) {
                     current_node = p_node.clone();
                     parent_node = BstNode::upgrade_weak_to_strong(current_node.borrow().parent.clone());
                 } else {
                     return Some(p_node.clone());
                 }
            } else if let Some(p_left) = &p_node.borrow().left {
                 if Rc::ptr_eq(&current_node, p_left) {
                      return Some(p_node.clone());
                 } else {
                     current_node = p_node.clone();
                     parent_node = BstNode::upgrade_weak_to_strong(current_node.borrow().parent.clone());
                 }

            } else {
                 current_node = p_node.clone();
                 parent_node = BstNode::upgrade_weak_to_strong(current_node.borrow().parent.clone());
            }
        }
        None
    }

    pub fn tree_insert(root: Option<BstNodeLink>, z_key: i32) -> BstNodeLink {
        let z_node = BstNode::new_bst_nodelink(z_key);
        let mut y: Option<BstNodeLink> = None;
        let mut x = root.clone();

        while let Some(current_x) = x.clone() {
            if current_x.borrow().key.unwrap() == z_key {
                println!("Key {} already exists, not inserting.", z_key);
                return root.unwrap();
            }

            y = Some(Rc::clone(&current_x));
            if z_key < current_x.borrow().key.unwrap() {
                x = current_x.borrow().left.clone();
            } else {
                x = current_x.borrow().right.clone();
            }
        }

        z_node.borrow_mut().parent = y.clone().map(|node| BstNode::downgrade(&node));

        if y.is_none() {
            z_node
        } else if z_key < y.as_ref().unwrap().borrow().key.unwrap() {
            y.unwrap().borrow_mut().left = Some(z_node.clone());
            root.unwrap()
        } else {
            y.unwrap().borrow_mut().right = Some(z_node.clone());
             root.unwrap()
        }
    }

    pub fn transplant(root: BstNodeLink, u: BstNodeLink, v: Option<BstNodeLink>) -> BstNodeLink {
        let u_parent = BstNode::upgrade_weak_to_strong(u.borrow().parent.clone());

        if u_parent.is_none() {
            if let Some(v_node) = v.clone() {
                v_node.borrow_mut().parent = None;
                v_node
            } else {
                 BstNode::new_bst_nodelink(-1)
            }
        } else if let Some(u_p) = u_parent {
             let mut u_p_mut = u_p.borrow_mut();
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
            }
            drop(u_p_mut);

            if let Some(v_node) = v {
                v_node.borrow_mut().parent = Some(BstNode::downgrade(&u_p));
            }

            root
        } else {
             root
        }
    }

    pub fn tree_delete(root: BstNodeLink, z: BstNodeLink) -> BstNodeLink {
        let z_borrowed = z.borrow();
        let z_left = z_borrowed.left.clone();
        let z_right = z_borrowed.right.clone();
        drop(z_borrowed);


        if z_left.is_none() {
             BstNode::transplant(root, Rc::clone(&z), z_right)
        } else if z_right.is_none() {
            BstNode::transplant(root, Rc::clone(&z), z_left)
        } else {
            let y = BstNode::minimum_nodelink(z_right.clone().unwrap());
            let y_parent = BstNode::upgrade_weak_to_strong(y.borrow().parent.clone());

            if !Rc::ptr_eq(&y_parent.as_ref().unwrap(), &z) {
                 let y_right = y.borrow().right.clone();
                 BstNode::transplant(root.clone(), Rc::clone(&y), y_right);

                y.borrow_mut().right = z_right.clone();
                if let Some(z_r) = z_right {
                    z_r.borrow_mut().parent = Some(BstNode::downgrade(&y));
                }

            }

            let new_root = BstNode::transplant(root, Rc::clone(&z), Some(Rc::clone(&y)));

            let z_left = z.borrow().left.clone().unwrap();
            y.borrow_mut().left = Some(z_left.clone());
            z_left.borrow_mut().parent = Some(BstNode::downgrade(&y));

            new_root
        }
    }

    #[allow(dead_code)]
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
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

    #[allow(dead_code)]
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    #[allow(dead_code)]
    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        anode.borrow().key == bnode.borrow().key
    }

    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        node.and_then(|weak_node| weak_node.upgrade())
    }
}
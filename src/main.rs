mod structure;
mod tool;

use crate::structure::bst::BstNode;
use crate::structure::tree::Node; 
use crate::structure::tree::NodeLink;
use crate::structure::bst::BstNodeLink;
use crate::tool::generate_dotfile; 
use crate::tool::generate_dotfile_bst;

// Import Rc
use std::rc::Rc;


fn main() {
    test_binary_search_tree();
}

fn test_binary_search_tree(){
    println!("--- Initial Tree Creation (using insert) ---");

    let mut rootlink: Option<BstNodeLink> = None;

    let values_to_insert = vec![15, 6, 18, 17, 20, 3, 7, 2, 4, 13, 9];

    for value in values_to_insert {
        rootlink = Some(BstNode::tree_insert(rootlink, value));
    }

    // Get the current root after insertions
    let mut current_root = rootlink.expect("Tree should not be empty after insertions");


    //print the tree at this time
    let main_tree_path_initial = "bst_graph_initial.dot";
    println!("Generating initial tree graph: {}", main_tree_path_initial);
    generate_dotfile_bst(&current_root, main_tree_path_initial);

    println!("\n--- Tree Search Tests ---");
    let search_keys = vec![15, 9, 22, 4, 100];

    for &key in search_keys.iter() {
        print!("tree search result of key {} is ", key);

        // Use the updated tree_search which takes BstNodeLink
        if let Some(node_result) = BstNode::tree_search(Rc::clone(&current_root), &key) {
            println!("found -> {:?}", node_result.borrow().key);
        } else {
            println!("not found");
        }
    }

    println!("\n--- Minimum/Maximum Tests ---");
    // Use the updated minimum/maximum functions which take BstNodeLink
    let min_node = BstNode::minimum_nodelink(Rc::clone(&current_root));
    println!("minimum result {:?}", min_node.borrow().key);

    let max_node = BstNode::maximum_nodelink(Rc::clone(&current_root));
    println!("maximum result {:?}", max_node.borrow().key);

    println!("\n--- Get Root Test ---");
    let root_node = BstNode::get_root(&max_node);
    println!("root node from max_node {:?}", root_node.borrow().key);
    let root_node_from_min = BstNode::get_root(&min_node);
     println!("root node from min_node {:?}", root_node_from_min.borrow().key);


    println!("\n--- Successor Tests ---");
    let query_keys = vec![
        2, // min_node, should return its parent Some(3)
        3, // should return 4
        4, // should return 6
        6, // should return 7
        7, // should return 9
        9, // should return 13
        13, // should return 15
        15, // root_node, should return the minimum of its right tree (17)
        17, // should return 18
        18, // should return 20
        20, // max_node, should return None
        22, // non-existent key
    ];

    for &key in query_keys.iter() {
        // Use the updated tree_search
        if let Some(node) = BstNode::tree_search(Rc::clone(&current_root), &key) {
            print!("successor of node ({}) is ", key);

            // Using the corrected tree_successor
            if let Some(successor) = BstNode::tree_successor(&node) {
                println!("{:?}", successor.borrow().key);
            } else {
                println!("not found");
            }
        } else {
            println!("node with key of {} does not exist, failed to get successor", key)
        }
    }

    println!("\n--- Insert Tests (Duplicates) ---");
    println!("Inserting 15 (already exists)...");
    // tree_insert will print a message if the key exists
    let new_root_after_insert_15 = BstNode::tree_insert(Some(Rc::clone(&current_root)), 15);
    let main_tree_path_insert_15 = "bst_graph_insert_15.dot";
    println!("Generating tree graph after inserting 15: {}", main_tree_path_insert_15);
    generate_dotfile_bst(&new_root_after_insert_15, main_tree_path_insert_15);
    // Rootlink should be the same as new_root_after_insert_15, no need to reassign unless the tree actually changed


    println!("\n--- Delete Tests ---");

    // Delete a leaf node (e.g., 4)
    println!("Deleting 4 (leaf node)...");
    if let Some(node_to_delete) = BstNode::tree_search(Rc::clone(&current_root), &4) {
        let new_root_after_delete_4 = BstNode::tree_delete(Rc::clone(&current_root), node_to_delete);
        let main_tree_path_delete_4 = "bst_graph_delete_4.dot";
        println!("Generating tree graph after deleting 4: {}", main_tree_path_delete_4);
        generate_dotfile_bst(&new_root_after_delete_4, main_tree_path_delete_4);
        rootlink = Some(new_root_after_delete_4); // Update rootlink
        current_root = rootlink.as_ref().unwrap().clone(); // Update current_root
    } else {
        println!("Node with key 4 not found.");
    }


    println!("Deleting 18 (node with one child)...");
     // Use the updated tree_search
     if let Some(node_to_delete) = BstNode::tree_search(Rc::clone(&current_root), &18) {
        let new_root_after_delete_18 = BstNode::tree_delete(Rc::clone(&current_root), node_to_delete);
        let main_tree_path_delete_18 = "bst_graph_delete_18.dot";
        println!("Generating tree graph after deleting 18: {}", main_tree_path_delete_18);
        generate_dotfile_bst(&new_root_after_delete_18, main_tree_path_delete_18);
        rootlink = Some(new_root_after_delete_18); // Update rootlink
        current_root = rootlink.as_ref().unwrap().clone(); // Update current_root
    } else {
        println!("Node with key 18 not found.");
    }


     println!("Deleting 6 (node with two children)...");
     // Use the updated tree_search
     if let Some(node_to_delete) = BstNode::tree_search(Rc::clone(&current_root), &6) {
        let new_root_after_delete_6 = BstNode::tree_delete(Rc::clone(&current_root), node_to_delete);
        let main_tree_path_delete_6 = "bst_graph_delete_6.dot";
        println!("Generating tree graph after deleting 6: {}", main_tree_path_delete_6);
        generate_dotfile_bst(&new_root_after_delete_6, main_tree_path_delete_6);
        rootlink = Some(new_root_after_delete_6); // Update rootlink
        current_root = rootlink.as_ref().unwrap().clone(); // Update current_root
    } else {
        println!("Node with key 6 not found.");
    }


    // Delete the root node (e.g., 15)
     println!("Deleting 15 (root node)...");
     // Use the updated tree_search
     if let Some(node_to_delete) = BstNode::tree_search(Rc::clone(&current_root), &15) {
        let new_root_after_delete_15 = BstNode::tree_delete(Rc::clone(&current_root), node_to_delete);
        let main_tree_path_delete_15 = "bst_graph_delete_15.dot";
        println!("Generating tree graph after deleting 15: {}", main_tree_path_delete_15);
        generate_dotfile_bst(&new_root_after_delete_15, main_tree_path_delete_15);
        rootlink = Some(new_root_after_delete_15); // Update rootlink
        current_root = rootlink.as_ref().unwrap().clone(); // Update current_root
    } else {
        println!("Node with key 15 not found.");
    }


    // Attempt to delete a non-existent node (e.g., 99)
    println!("Attempting to delete 99 (non-existent)...");
     if let Some(node_to_delete) = BstNode::tree_search(Rc::clone(&current_root), &99) {
        println!("Node with key 99 found (unexpected), proceeding with delete.");
        let new_root_after_delete_99 = BstNode::tree_delete(Rc::clone(&current_root), node_to_delete);
        let main_tree_path_delete_99 = "bst_graph_delete_99.dot";
        println!("Generating tree graph after deleting 99: {}", main_tree_path_delete_99);
        generate_dotfile_bst(&new_root_after_delete_99, main_tree_path_delete_99);
        rootlink = Some(new_root_after_delete_99);
        current_root = rootlink.as_ref().unwrap().clone(); 

    } else {
        println!("Node with key 99 not found, cannot delete.");
    }


     println!("\n--- Final Tree State ---");
     let main_tree_path_final = "bst_graph_final.dot";
     println!("Generating final tree graph: {}", main_tree_path_final);
     generate_dotfile_bst(&current_root, main_tree_path_final);

}

#[allow(dead_code)]
fn test_binary_tree() {
    //create the nodelink of the root node
    let rootlink: NodeLink = Node::new_nodelink(5);

    //add a new left node value
    rootlink.borrow_mut().add_left_child(&rootlink, 3);
    //add a new right node value
    rootlink.borrow_mut().add_right_child(&rootlink, 7);

    let mut main_tree_path = "prime.dot";
    generate_dotfile(&rootlink, main_tree_path);

    let left_subtree = &rootlink.borrow().left;
    if let Some(left_tree_extract) = left_subtree {
        left_tree_extract
            .borrow_mut()
            .add_left_child(left_tree_extract, 2);
        left_tree_extract
            .borrow_mut()
            .add_right_child(left_tree_extract, 4);
    }

    let right_subtree = &rootlink.borrow().right;
    if let Some(right_tree_extract) = right_subtree {
        right_tree_extract
            .borrow_mut()
            .add_right_child(right_tree_extract, 10);
    }

    main_tree_path = "prime_t2.dot";
    generate_dotfile(&rootlink, main_tree_path);

    let recorded_depth = rootlink.borrow().tree_depth();
    println!("Current tree depth: {0}", recorded_depth);

    let total_nodes = rootlink.borrow().count_nodes();
    println!("Amount of nodes in current tree: {0}", total_nodes);

    //Call count_nodes_by_nodelink function, supplied right subtree as parameter
    //TODO
    let subtree_count = Node::count_nodes_by_nodelink(&right_subtree.clone().unwrap(), 0);
    println!("Amount of nodes in current subtree: {0}", subtree_count);

    let _left_subtree_sibling = Node::get_sibling(&left_subtree.as_ref().unwrap());

    let left_subtree = rootlink.borrow().get_node_by_value(3);
    println!("left subtree seek by value {:?}", left_subtree);

    let another_left_subtree = rootlink
        .borrow()
        .get_node_by_full_property(&left_subtree.as_ref().unwrap());
    println!(
        "left subtree seek by full property {:?}",
        another_left_subtree
    );

    let rootlink2 = rootlink.borrow().get_nodelink_copy();

    let flag = rootlink2.borrow_mut().discard_node_by_value(3);
    println!("status of node deletion: {0}", flag);

    //print the tree again
    main_tree_path = "prime_t3.dot";
    generate_dotfile(&rootlink2, main_tree_path);

    //TODO
    let depth_now = rootlink2.borrow().tree_depth();
    println!("Depth after discard {0}", depth_now);

    let count_now = rootlink2.borrow().count_nodes();
    println!("Count nodes after discard {0}", count_now);

    //print the tree again
    main_tree_path = "prime_t4.dot";
    generate_dotfile(&rootlink, main_tree_path);
}
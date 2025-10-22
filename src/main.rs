use std::{
    collections::VecDeque,
    env::{self},
    fs, io,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BinaryTree {
    root: Option<Box<Node>>,
}

#[derive(Serialize, Deserialize)]
struct Node {
    /*Comment for future me:
    Box is for putting Node to heap to resolve infitie space problem
    Option is for resolve None child problem
    or i think so
    */
    value: isize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl BinaryTree {
    //Creating empty node
    fn new() -> Self {
        BinaryTree { root: None }
    }

    //Base tree_insert function to invoke recursive one
    fn tree_insert(&mut self, value: isize) {
        Self::insert_recursive(&mut self.root, value);
    }

    //Recursive function to reivoke it every time, nodes are not empty
    fn insert_recursive(node_pointer: &mut Option<Box<Node>>, value: isize) {
        //checking if value of node are emtpty or no
        match node_pointer {
            Some(current_node) => {
                if current_node.value == value {
                    //in this implementation duplaicates are not allowed
                    println!("Duplicates are exluded!");
                    return;
                } else if current_node.value > value {
                    //invoking function but instead of current node with left node
                    Self::insert_recursive(&mut current_node.left, value);
                } else {
                    //same but with right node
                    Self::insert_recursive(&mut current_node.right, value);
                }
            }
            None => {
                let new_node = Box::new(Node {
                    value: value,
                    left: None,
                    right: None,
                });
                //adding new node to our node pointer
                *node_pointer = Some(new_node);
            }
        }
    }

    // fn show_left_first(&self) {
    //     let mut node_stack = vec![&self.root];
    //     loop {
    //         if node_stack.len() == 0 {
    //             break;
    //         }
    //         let link = node_stack[node_stack.len() - 1];
    //         match link {
    //             Some(current_node) => {
    //                 print!("-{}-", current_node.value);
    //                 node_stack.pop();
    //                 if !current_node.right.is_none() {
    //                     node_stack.push(&current_node.right);
    //                 }
    //                 if !current_node.left.is_none() {
    //                     node_stack.push(&current_node.left);
    //                 }
    //             }
    //             None => {
    //                 //condition if tree is empty, otherwise would be inifite loop
    //                 node_stack.pop();
    //             }
    //         }
    //     }
    //     println!()
    // }

    // fn show_by_rows(&self) {
    //     if self.root.is_none() {
    //         println!("Empty tree");
    //         return;
    //     }
    //     // vecdeque is queue in input on back and ouput on front
    //     let mut queue = VecDeque::new();

    //     queue.push_back(&self.root);
    //     loop {
    //         if queue.len() == 0 {
    //             break;
    //         }
    //         for _ in 0..queue.len() {
    //             if let Some(Some(node)) = queue.pop_front() {
    //                 print!("{} ", node.value);
    //                 if !node.left.is_none() {
    //                     queue.push_back(&node.left);
    //                 }
    //                 if !node.right.is_none() {
    //                     queue.push_back(&node.right);
    //                 }
    //             }
    //         }
    //         println!();
    //     }
    // }

    fn show_pretty(&self) {
        // change this to loop for every value not only in queue for each level, add none to vector and then print all tree with nones and not
        if self.root.is_none() {
            println!("Empty tree");
            return;
        }
        // vecdeque is queue in input on back and ouput on front
        let mut queue: VecDeque<Option<&Box<Node>>> = VecDeque::new();
        let mut node_values = VecDeque::new();
        //must be float to calcualte log2
        let mut node_counter: f64 = 0.0;
        let mut level_size = 1;

        queue.push_back(self.root.as_ref());
        loop {
            if queue.len() == 0 {
                break;
            }

            let mut next_level_check_not_none_only = false;
            for _ in 0..level_size {
                if let Some(option_node) = queue.pop_front() {
                    match option_node {
                        Some(node) => {
                            node_values.push_back(Some(node.value));
                            queue.push_back(node.left.as_ref());
                            queue.push_back(node.right.as_ref());
                            if !node.left.is_none() || !node.right.is_none() {
                                next_level_check_not_none_only = true;
                            }
                        }
                        None => {
                            node_values.push_back(None);
                            queue.push_back(None);
                            queue.push_back(None);
                        }
                    }
                }
                node_counter += 1.0;
            }
            level_size *= 2;
            if !next_level_check_not_none_only {
                break;
            }
        }

        let tree_height = node_counter.log2() + 1.0;
        let tree_height_u32 = tree_height.floor() as u32;

        println!();
        let mut values_index = 0;
        for level in 0..tree_height_u32 {
            let nodes_at_level = 2usize.pow(level);
            let spacing_between = 2usize.pow(tree_height_u32 - level) - 1;
            let leading_spaces = 2usize.pow(tree_height_u32 - level - 1) - 1;

            for _ in 0..leading_spaces {
                print!(" ");
            }

            for i in 0..nodes_at_level {
                if values_index < node_values.len() {
                    if let Some(val) = node_values[values_index] {
                        print!("{}", val);
                    } else {
                        print!("N");
                    }
                    values_index += 1;
                }

                if i < nodes_at_level - 1 {
                    for _ in 0..spacing_between {
                        print!(" ");
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(filename, json)?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> io::Result<Self> {
        let json = fs::read_to_string(filename)?;
        let tree = serde_json::from_str(&json)?;
        Ok(tree)
    }
}

fn main() {
    let tree_file = "tree.json";
    let mut my_tree = BinaryTree::load_from_file(tree_file).unwrap_or_else(|_| BinaryTree::new());

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Use command:");
        println!("./tree <option>");
        println!("Use './tree help' to get all options");
        return;
    }

    match args[1].as_str() {
        "help" => {
            println!("'./tree new' to add new tree (clear already existing one)");
            println!("'./tree add <values...>' to insert value or values to tree");
            println!("'./tree show' to display tree");
            println!("'./tree clear' to clear current tree");
            return;
        }
        "new" => {
            my_tree = BinaryTree::new();
            println!("New tree created!")
        }
        "add" => {
            let mut count = 0;
            if args.len() < 3 {
                println!("'./tree add <values...>' to insert value or values to the tree");
                return;
            }
            for value_str in &args[2..] {
                match value_str.parse::<isize>() {
                    Ok(value) => {
                        count += 1;
                        my_tree.tree_insert(value);
                    }
                    Err(_) => {
                        eprintln!("Error: {} is not a valid number", value_str);
                    }
                }
            }
            println!("Added {} numbers", count);
        }
        "show" => {
            if args.len() != 2 {
                println!("'./tree show' to display tree");
                return;
            }
            my_tree.show_pretty()
        }

        "clear" => {
            if let Err(e) = fs::remove_file(tree_file){
                eprintln!("Error clearning file: {}", e);
            } else{
                println!("Tree cleaned");
            }
            return;
        }
        _ => {
            println!("Wrong option!");
            println!("Use command:");
            println!("./tree <option>");
            println!("Use './tree help' to get all options");
        }
    }
    if let Err(e) = my_tree.save_to_file(tree_file){
        eprintln!("Error saving tree to file: {}", e);
    } else{
        // println!("Tree saved!");
    }
}

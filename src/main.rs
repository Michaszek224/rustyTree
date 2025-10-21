#[derive(Debug)]
struct BinaryTree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
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

    fn show_left_first(&self) {
        let mut node_stack = vec![&self.root];
        loop {
            if node_stack.len() == 0 {
                break;
            }
            let link = node_stack[node_stack.len() - 1];
            match link {
                Some(current_node) => {
                    print!("-{}-", current_node.value);
                    node_stack.pop();
                    if !current_node.right.is_none() {
                        node_stack.push(&current_node.right);
                    }
                    if !current_node.left.is_none() {
                        node_stack.push(&current_node.left);
                    }
                }
                None => {
                    //condition if tree is empty, otherwise would be inifite loop
                    node_stack.pop();
                }
            }
        }
        println!()
    }
}

fn main() {
    let mut my_tree = BinaryTree::new();
    my_tree.tree_insert(11);
    my_tree.tree_insert(9);
    my_tree.tree_insert(15);
    my_tree.tree_insert(7);
    my_tree.tree_insert(10);
    my_tree.tree_insert(14);
    my_tree.tree_insert(18);
    my_tree.tree_insert(3);
    my_tree.show_left_first();
}

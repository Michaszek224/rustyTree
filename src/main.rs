#[derive(Debug)]
struct BinaryTree{
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
    fn new() -> Self{
        BinaryTree { root: None }
    }

    fn insert(&mut self, value: isize){
        let new_node = Box::new(Node{
            value: value,
            left: None,
            right: None,
        });

        if self.root.is_none(){
            self.root = Some(new_node);
            return
        }
    }
}

fn main() {
    let mut my_tree = BinaryTree::new();
    my_tree.insert(12);
    println!("{:?}", my_tree);
}

#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}


struct TreeIter<'a, T>{
    unvisited: Vec<&'a TreeNode<T>>
}

impl <'a, T: 'a> TreeIter<'a, T> {

    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>){
        while let BinaryTree::NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree= &node.left;
        }
    }

}

impl <T> BinaryTree<T>{
    fn iter(&self) -> TreeIter<'_, T>{
        let mut iter= TreeIter {unvisited: Vec::new()};
        iter.push_left_edge(self);
        iter
    }
}

impl <'a, T: 'a> IntoIterator for &'a BinaryTree<T>{
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl <'a, T> Iterator for TreeIter<'a, T>{ 

    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let node= self.unvisited.pop()?;

        self.push_left_edge(&node.right);

        Some(&node.element)
    }
    
}

fn main(){

    let mut tree= BinaryTree::Empty;
    tree.add("jaeger");
    tree.add("robot");
    tree.add("droid");
    tree.add("mecha");

    let mut v= Vec::new();
    for kind in &tree{
        v.push(*kind);
    }

    assert_eq!(v, ["droid", "jaeger", "mecha", "robot"]);
    
    assert_eq!(
        tree.iter()
        .map(|name| format!("mega-{}", name))
        .collect::<Vec<_>>(),
        vec!["mega-droid", "mega-jaeger", "mega-mecha", "mega-robot"]
    );
    
    dbg!(tree);
    
    println!("✅ Finalizado!");
    
}
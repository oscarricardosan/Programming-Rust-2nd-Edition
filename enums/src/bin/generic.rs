#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
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

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

fn main() {
    let jupiter_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));
    let mercury_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let mars_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree,
    }));

    let venus_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Venus",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let uranus_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: BinaryTree::Empty,
        right: venus_tree,
    }));

    let jupiter_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Saturno",
        left: mars_tree,
        right: uranus_tree,
    }));

    dbg!(jupiter_tree);

    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Luna");
    tree.add("Venus");
    dbg!(tree);
    println!("Terminado");
}

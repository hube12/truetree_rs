use core::fmt;
use std::borrow::BorrowMut;
use std::cmp::max;
use std::fmt::{Debug, Display};
use std::mem::{replace, swap};
use std::ops::Not;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Side {
    Left = 0,
    Right = 1,
}

impl Not for Side {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Node<T: Clone + Ord + Eq + Debug + Display> {
    children: [Tree<T>; 2],
    value: T,
    height: usize,
}

impl<T: Clone + Ord + Eq + Debug + Display> fmt::Pointer for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr = self as *const Self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

impl<T: Clone + Ord + Eq + Debug + Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

type Tree<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Clone)]
pub struct AvlTree<T: Clone + Ord + Eq + Debug + Display> {
    root: Tree<T>,
}

#[cfg(test)]
mod test_tree {
    use super::*;

    const TEST_1: u64 = 42;
    const TEST_2: u64 = 420;
    const TEST_3: u64 = 66;
    const TEST_4: u64 = 88;
    const TEST_5: u64 = 99;

    #[test]
    fn test_create() {
        let tree: AvlTree<u64> = AvlTree::new();
        assert!(tree.root.is_none());
        assert!(tree.is_empty())
    }

    #[test]
    fn test_insert() {
        let mut tree: AvlTree<u64> = AvlTree::new();
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_1);
        assert!(res.is_ok());
        assert!(!tree.is_empty());
        assert!(tree.root.is_some());
        assert_eq!(tree.root.unwrap().value, TEST_1);
    }

    #[test]
    fn test_insert_twice() {
        let mut tree: AvlTree<u64> = AvlTree::new();
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_1);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        assert!(!tree.is_empty());
        assert!(tree.root.is_some());
        assert_eq!(tree.root.as_ref().unwrap().value, TEST_1);
        assert!(tree.root.as_ref().unwrap().children[Side::Right as usize].is_some());
        assert!(tree.root.as_ref().unwrap().children[Side::Left as usize].is_none());
        assert_eq!(tree.root.as_ref().unwrap().children[Side::Right as usize].as_ref().unwrap().value, TEST_2);
    }

    #[test]
    fn test_insert_thrice() {
        let mut tree: AvlTree<u64> = AvlTree::new();
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_1);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        assert!(!tree.is_empty());
        assert!(tree.root.is_some());
        assert_eq!(tree.root.as_ref().unwrap().value, TEST_2);
        assert!(tree.root.as_ref().unwrap().children[Side::Right as usize].is_some());
        assert_eq!(tree.root.as_ref().unwrap().children[Side::Right as usize].as_ref().unwrap().value, TEST_2);
        assert!(tree.root.as_ref().unwrap().children[Side::Left as usize].is_some());
        assert_eq!(tree.root.as_ref().unwrap().children[Side::Left as usize].as_ref().unwrap().value, TEST_1);
    }

    #[test]
    fn test_contains(){
        let mut tree:AvlTree<u64>=AvlTree::new();
        tree.insert(&TEST_1).expect("Insert failed")
            .insert(&TEST_2).expect("Insert failed")
            .insert(&TEST_3).expect("Insert failed")
            .insert(&TEST_4).expect("Insert failed");

        assert!(tree.contains(&TEST_1));
        assert!(tree.contains(&TEST_2));
        assert!(tree.contains(&TEST_3));
        assert!(tree.contains(&TEST_4));
        assert!(!tree.contains(&TEST_5));
        tree.insert(&TEST_5).expect("Insert failed");
        assert!(tree.contains(&TEST_5));
    }

    #[test]
    fn test_insert_complex() {
        let mut tree: AvlTree<u64> = AvlTree::new();
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_1);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        assert!(!tree.is_empty());
        assert_eq!(tree.root.unwrap().height, 4)
    }


    #[test]
    fn test_insert_heap_var() {
        let mut tree: AvlTree<u64> = AvlTree::new();
        {
            let value: u64 = 67;
            let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&value);
            assert!(res.is_ok());
            drop(value);
        }
        assert!(!tree.is_empty());
        assert_eq!(tree.root.unwrap().value, 67)
    }

    #[test]
    fn test_delete_heap() {
        let mut tree: AvlTree<u64> = AvlTree::new();
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_1);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&TEST_2);
        assert!(res.is_ok());
        assert!(!tree.is_empty());
        assert_eq!(tree.root.as_ref().unwrap().value, TEST_2);
        tree.clear();
        assert_eq!(tree.depth(), 0);
        tree.delete();
    }

    #[test]
    fn test_delete_complex() {
        let mut tree: AvlTree<u64> = AvlTree::new();
        {
            let payload: u64 = 67;
            let res: Result<&mut AvlTree<u64>, &str> = tree.insert(&payload);
            assert!(res.is_ok());
            drop(payload);
        }
        assert!(!tree.is_empty());
        assert_eq!(tree.root.as_ref().unwrap().value, 67);
        tree.delete();
    }

    #[test]
    fn test_height() {
        let mut tree: AvlTree<u64> = AvlTree::new();
        let _res = tree.insert(&TEST_1).expect("Failed insert")
            .insert(&TEST_2).expect("Failed insert")
            .insert(&TEST_3).expect("Failed insert")
            .insert(&TEST_4).expect("Failed insert")
            .insert(&TEST_5).expect("Failed insert");
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn test_balancing() {
        let tree: AvlTree<u64> = AvlTree::new();
        assert!(tree.is_balanced());
        tree.delete();
        let mut tree: AvlTree<u64> = AvlTree::new();
        let _res = tree.insert(&TEST_1).expect("Failed insert")
            .insert(&TEST_2).expect("Failed insert")
            .insert(&TEST_3).expect("Failed insert")
            .insert(&TEST_4).expect("Failed insert")
            .insert(&TEST_5).expect("Failed insert");
        assert!(tree.is_correct());
        assert!(tree.is_balanced());
    }

    #[test]
    fn test_remove() {
        let mut tree: AvlTree<u64> = AvlTree::new();
        let _res = tree.insert(&TEST_1).expect("Failed insert")
            .insert(&TEST_2).expect("Failed insert")
            .insert(&TEST_3).expect("Failed insert")
            .insert(&TEST_4).expect("Failed insert")
            .insert(&TEST_5).expect("Failed insert");
        assert!(tree.is_correct());
        assert!(tree.is_balanced());
        assert_eq!(tree.height(), 3);
        let res: Result<&mut AvlTree<u64>, &str> = tree.remove(&TEST_1);
        assert!(res.is_ok());
        assert_eq!(tree.height(), 3);
        assert!(tree.is_balanced());
        let res: Result<&mut AvlTree<u64>, &str> = tree.remove(&TEST_2);
        assert!(res.is_ok());
        assert_eq!(tree.height(), 2);
        assert!(tree.is_balanced());
        let res: Result<&mut AvlTree<u64>, &str> = tree.remove(&TEST_2);
        assert!(res.is_err());
        assert_eq!(tree.height(), 2);
        assert!(tree.is_balanced());
        let _res = tree.remove(&TEST_3).expect("Failed removed");
        let _res = tree.remove(&TEST_4).expect("Failed removed");
        let _res = tree.remove(&TEST_5).expect("Failed removed");
        assert_eq!(tree.height(), 0);
    }

    #[test]
    fn test_min_max() {
        let mut tree: AvlTree<u64> = AvlTree::new();
        let _res = tree.insert(&TEST_1).expect("Failed insert")
            .insert(&TEST_2).expect("Failed insert")
            .insert(&TEST_3).expect("Failed insert")
            .insert(&TEST_4).expect("Failed insert")
            .insert(&TEST_5).expect("Failed insert");
        assert!(tree.min().is_some());
        assert_eq!(tree.min().unwrap(), TEST_1);
        assert!(tree.max().is_some());
        assert_eq!(tree.max().unwrap(), TEST_2);
    }

    #[test]
    fn test_width_count_depth() {
        let mut tree: AvlTree<u64> = AvlTree::new();
        let _res = tree.insert(&TEST_1).expect("Failed insert")
            .insert(&TEST_2).expect("Failed insert")
            .insert(&TEST_3).expect("Failed insert")
            .insert(&TEST_4).expect("Failed insert")
            .insert(&TEST_5).expect("Failed insert");
        assert_eq!(tree.depth(), 3);
        assert_eq!(tree.width(), 3);
        assert_eq!(tree.count(), 5);
    }
}

/// This is a balanced tree implementation (also called as AVL tree)
/// We allow you to define a custom type T to pass a the tree payload
/// This payload should have the clone, ord, eq, debug and display trait derived
/// The Ord trait is used to insert the values and to get it (this allow to match only partial payload)
/// The Eq trait is used to remove a node
/// The Clone trait is used to copy the value inside the tree
/// The Display trait is used to print/dump the tree
impl<'a, T: 'a + Clone + Ord + Eq + Debug + Display> AvlTree<T> {
    /// Create a new tree (a tree is defined as an optional heap pointer to a Node of type T)
    /// By default we made a wrapper around the internal implementation with a root node
    pub fn new() -> Self {
        AvlTree {
            root: None
        }
    }

    /// Create a new tree (a tree is defined as an optional heap pointer to a Node of type T)
    /// with a default value, all value passed are cloned and should implement Clone, Ord, Eq and Debug
    /// There is an example of such Payload creation in integration_avl
    pub fn with(value: &T) -> Self {
        AvlTree {
            root: Node::create_tree(value)
        }
    }

    /// Insert a value in the tree and return itself if no errors (by default we allow duplicate key value (using Eq trait, not Ord)
    /// You can chain multiple insert
    pub fn insert(&mut self, value: &T) -> Result<&mut Self, &str> {
        if self.root.is_none() {
            self.root = Node::create_tree(value)
        } else {
            if !self.root.as_mut().unwrap().insert(value.clone()) {
                return Err("Can not insert same value twice");
            }
        }
        Ok(&mut *self)
    }

    /// Delete all the values in the tree, the structure holding the tree should theoretically not be reused
    /// This is effectively the same as clear()
    pub fn delete(mut self) -> () {
        if !self.root.is_none() {
            Node::delete(self.root.borrow_mut())
        }
        drop(self);
    }

    /// Delete all the values in the tree, the structure holding the tree can be reused afterwards
    /// This is effectively the same as delete()
    pub fn clear(&mut self) -> () {
        if !self.root.is_none() {
            Node::delete(self.root.borrow_mut())
        }
    }
    /// Remove a value from the tree and return itself if was successful else return an error
    /// Warning we use Eq to remove the correct value, if you don't know all the fields, use the get which use Ord only
    /// See integration for an example.
    pub fn remove(&mut self, value: &T) -> Result<&mut Self, &str> {
        if self.root.is_none() {
            Err("You don't have any node in the tree")
        } else {
            let res: Option<T> = Node::remove(&mut self.root, value);
            if res.is_none() {
                Err("The value was not found")
            } else if &res.unwrap() != value {
                Err("ERROR ! The value removed was not the correct one.")
            } else {
                Ok(&mut *self)
            }
        }
    }

    /// Print the tree as a JSON formatted string,
    /// It's possible to prettify it with the boolean
    pub fn print(&self, prettify: bool) -> () {
        if self.root.is_none() {
            println!("You don't have any node in the tree");
        } else {
            println!("{}", self.root.as_ref().unwrap().dump(prettify));
        }
    }

    /// Dump the tree as a JSON formatted string,
    /// It's possible to prettify it with the boolean
    pub fn dump(&self, prettify: bool) -> Result<String, &str> {
        if self.root.is_none() {
            Err("You don't have any node in the tree")
        } else {
            Ok(self.root.as_ref().unwrap().dump(prettify))
        }
    }

    /// Get a value based only on Ord (not Eq), this allow loosy check in case of complex payload
    /// This return only the value or none, for a subtree see find
    pub fn get(&self, value: &T) -> Option<T> {
        let tree: &Tree<T> = Node::get(&self.root, value);
        return tree.as_ref().map_or(None, |x| Some(x.value.clone()));
    }

    /// Find a value and return it as the subtree (equivalent as get but allow to chain operation on the subtree)
    pub fn find(&self, value: &T) -> Result<Self, &str> {
        let tree: &Tree<T> = Node::get(&self.root, value);
        if tree.is_none() {
            return Err("Not found");
        }
        return Ok(AvlTree {
            root: tree.clone()
        });
    }

    /// Check if a value is contained in the tree with Ord trait only
    pub fn contains(&self, value: &T) -> bool {
        Node::get(&self.root, value).is_some()
    }

    /// Check if a value is contained in the tree
    pub fn contains_exact(&self, value: &T) -> bool {

        let res=Node::get(&self.root, value);
        if res.is_none(){
            return false;
        }
        return &res.as_ref().unwrap().value==value;
    }

    /// Check if the tree is empty or not
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Failing case for test
    pub fn fail(&mut self) -> Result<&mut Self, &str> {
        Err("Oups, I always fail")
    }

    /// Working case for test
    pub fn works(&mut self) -> Result<&mut Self, &str> {
        Ok(&mut *self)
    }

    /// Return the height of the tree as a fast heuristic (this could be wrong if you tampered the tree)
    pub fn height(&self) -> usize {
        self.root.as_ref().map_or(0, |node: &Box<Node<T>>| node.height())
    }

    /// Return the true depth of the tree by going through all the nodes
    pub fn depth(&self) -> usize {
        self.root.as_ref().map_or(0, |node: &Box<Node<T>>| node.depth())
    }
    /// Get the number of leaves, a leave is a node which has one or more children missing (so a node with only one child is a leave also)
    ///     1          1
    ///      \        / \
    ///       2      2   3
    /// Those have a width of 2
    pub fn width(&self) -> usize {
        self.root.as_ref().map_or(0, |node: &Box<Node<T>>| node.width())
    }

    /// Get the number of nodes in the tree
    pub fn count(&self) -> usize {
        self.root.as_ref().map_or(0, |node: &Box<Node<T>>| node.count())
    }

    /// Get the minimum of the tree (or the left most)
    pub fn min(&self) -> Option<T> {
        self.root.as_ref().map_or(None, |node: &Box<Node<T>>| Some(node.min().clone()))
    }

    /// Get the maximum of the tree (or the right most)
    pub fn max(&self) -> Option<T> {
        self.root.as_ref().map_or(None, |node: &Box<Node<T>>| Some(node.max().clone()))
    }

    /// Check if the tree is balanced (this is quite intensive but gave correct result)
    pub fn is_balanced(&self) -> bool {
        self.root.as_ref().map_or(true, |node: &Box<Node<T>>| node.is_balanced())
    }

    /// Check if the heights are correct (might have not be registered correctly, this is a soft check)
    pub fn is_correct(&self) -> bool {
        self.root.as_ref().map_or(true, |node: &Box<Node<T>>| node.sanity_check())
    }
}

#[cfg(test)]
mod test_node {
    use super::*;

    const TEST_1: u64 = 42;
    const TEST_2: u64 = 420;
    const TEST_3: u64 = 66;
    const TEST_4: u64 = 88;
    const TEST_5: u64 = 99;

    #[test]
    fn test_side() {
        assert_eq!(Side::Left as u8, 0);
        assert_eq!(Side::Right as u8, 1);
        assert_eq!(!Side::Left, Side::Right);
        assert_eq!(!Side::Right, Side::Left);
        assert_eq!(!!Side::Right, Side::Right);
    }

    #[test]
    fn test_create() {
        let tree: Option<Box<Node<u64>>> = Some(Box::new(Node {
            children: [
                Some(Box::new(Node {
                    children: [None, None],
                    value: TEST_2,
                    height: 1,
                })),
                None
            ],
            value: TEST_1,
            height: 2,
        }));
        let ref_tree: Option<&Box<Node<u64>>> = tree.as_ref();
        assert!(ref_tree.is_some());
        assert_eq!(ref_tree.unwrap().height, 2);
        assert_eq!(ref_tree.unwrap().value, TEST_1);
        assert_eq!(ref_tree.unwrap().children.len(), 2);
        assert_eq!(ref_tree.unwrap().children[1], None);
        assert!(ref_tree.unwrap().children[0].is_some());
        assert_eq!(ref_tree.unwrap().children[0].as_ref().unwrap().value, TEST_2);
        assert_eq!(ref_tree.unwrap().children[0].as_ref().unwrap().height, 1);
    }

    #[test]
    fn test_sanity_check() {
        let tree: Option<Box<Node<u64>>> = Some(Box::new(Node {
            children: [
                Some(Box::new(Node {
                    children: [None, None],
                    value: TEST_2,
                    height: 1,
                })),
                Some(Box::new(Node {
                    children: [None, None],
                    value: TEST_3,
                    height: 1,
                })),
            ],
            value: TEST_1,
            height: 2,
        }));
        let ref_tree: Option<&Box<Node<u64>>> = tree.as_ref();
        assert!(ref_tree.is_some());
        assert_eq!(ref_tree.unwrap().sanity_check(), true);
        assert_eq!(ref_tree.unwrap().is_balanced(), true);
        let tree: Option<Box<Node<u64>>> = Some(Box::new(Node {
            children: [
                Some(Box::new(Node {
                    children: [None, None],
                    value: TEST_2,
                    height: 1,
                })),
                Some(Box::new(Node {
                    children: [None, None],
                    value: TEST_3,
                    height: 2,// incorrect height here (will fail sanity but not balanced)
                })),
            ],
            value: TEST_1,
            height: 2,
        }));
        let ref_tree: Option<&Box<Node<u64>>> = tree.as_ref();
        assert!(ref_tree.is_some());
        assert_eq!(ref_tree.unwrap().sanity_check(), false);
        assert_eq!(ref_tree.unwrap().is_balanced(), true);
        let tree: Option<Box<Node<u64>>> = Some(Box::new(Node {
            children: [
                Some(Box::new(Node {
                    children: [Some(Box::new(Node {
                        children: [None, None],
                        value: TEST_3,
                        height: 1,
                    })), None],
                    value: TEST_2,
                    height: 2,
                })),
                None,
            ],
            value: TEST_1,
            height: 3,
        }));
        let ref_tree: Option<&Box<Node<u64>>> = tree.as_ref();
        assert!(ref_tree.is_some());
        assert_eq!(ref_tree.unwrap().sanity_check(), true);
        assert_eq!(ref_tree.unwrap().is_balanced(), false);
        assert_eq!(ref_tree.unwrap().depth(), ref_tree.unwrap().height);

        let tree: Option<Box<Node<u64>>> = Some(Box::new(Node {
            children: [
                Some(Box::new(Node {
                    children: [Some(Box::new(Node {
                        children: [None, None],
                        value: TEST_3,
                        height: 1,
                    })), None],
                    value: TEST_2,
                    height: 22121,
                })),
                None,
            ],
            value: TEST_1,
            height: 3,
        }));
        let ref_tree: Option<&Box<Node<u64>>> = tree.as_ref();
        assert!(ref_tree.is_some());
        assert_eq!(ref_tree.unwrap().sanity_check(), false);
        assert_eq!(ref_tree.unwrap().is_balanced(), false);
        assert_eq!(ref_tree.unwrap().depth(), ref_tree.unwrap().height)
    }

    #[test]
    ///           1              2
    ///         /  \            / \
    ///        2    3    ->    4   1
    ///       / \                 / \
    ///      4   5               5   3
    ///
    ///
    fn test_rotate_right() {
        let mut tree: Option<Box<Node<u64>>> = Some(Box::new(Node {
            children: [
                Some(Box::new(Node {
                    children: [Some(Box::new(Node {
                        children: [None, None],
                        value: TEST_4,
                        height: 1,
                    })), Some(Box::new(Node {
                        children: [None, None],
                        value: TEST_5,
                        height: 1,
                    }))],
                    value: TEST_2,
                    height: 2,
                })),
                Some(Box::new(Node {
                    children: [None, None],
                    value: TEST_3,
                    height: 1,
                })),
            ],
            value: TEST_1,
            height: 3,
        }));
        let ref_tree: &Box<Node<u64>> = tree.as_ref().unwrap();
        assert_eq!(ref_tree.value, TEST_1);
        assert_eq!(ref_tree.children[Side::Left as usize].as_ref().unwrap().value, TEST_2);
        assert_eq!(ref_tree.children[Side::Left as usize].as_ref().unwrap().children[Side::Left as usize].as_ref().unwrap().value, TEST_4);
        assert_eq!(ref_tree.children[Side::Left as usize].as_ref().unwrap().children[Side::Right as usize].as_ref().unwrap().value, TEST_5);
        assert_eq!(ref_tree.children[Side::Right as usize].as_ref().unwrap().value, TEST_3);
        let res: bool = tree.as_mut().unwrap().rotate(Side::Right);
        assert!(res);
        let ref_tree: &Box<Node<u64>> = tree.as_ref().unwrap();
        assert_eq!(ref_tree.value, TEST_2);
        assert_eq!(ref_tree.children[Side::Left as usize].as_ref().unwrap().value, TEST_4);
        assert_eq!(ref_tree.children[Side::Right as usize].as_ref().unwrap().value, TEST_1);
        assert_eq!(ref_tree.children[Side::Right as usize].as_ref().unwrap().children[Side::Left as usize].as_ref().unwrap().value, TEST_5);
        assert_eq!(ref_tree.children[Side::Right as usize].as_ref().unwrap().children[Side::Right as usize].as_ref().unwrap().value, TEST_3);
    }

    #[test]
    ///           3              1
    ///          / \            / \
    ///         1   5    <-    2   3
    ///        / \                / \
    ///       2   4              4   5
    ///
    ///
    fn test_rotate_left() {
        let mut tree: Option<Box<Node<u64>>> = Some(Box::new(Node {
            children: [
                Some(Box::new(Node {
                    children: [None, None],
                    value: TEST_2,
                    height: 1,
                })),
                Some(Box::new(Node {
                    children: [Some(Box::new(Node {
                        children: [None, None],
                        value: TEST_4,
                        height: 1,
                    })), Some(Box::new(Node {
                        children: [None, None],
                        value: TEST_5,
                        height: 1,
                    }))],
                    value: TEST_3,
                    height: 2,
                })),
            ],
            value: TEST_1,
            height: 3,
        }));
        let ref_tree: &Box<Node<u64>> = tree.as_ref().unwrap();
        assert_eq!(ref_tree.value, TEST_1);
        assert_eq!(ref_tree.children[Side::Left as usize].as_ref().unwrap().value, TEST_2);
        assert_eq!(ref_tree.children[Side::Right as usize].as_ref().unwrap().value, TEST_3);
        assert_eq!(ref_tree.children[Side::Right as usize].as_ref().unwrap().children[Side::Left as usize].as_ref().unwrap().value, TEST_4);
        assert_eq!(ref_tree.children[Side::Right as usize].as_ref().unwrap().children[Side::Right as usize].as_ref().unwrap().value, TEST_5);
        let res: bool = tree.as_mut().unwrap().rotate(Side::Left);
        assert!(res);
        let ref_tree: &Box<Node<u64>> = tree.as_ref().unwrap();
        assert_eq!(ref_tree.value, TEST_3);
        assert_eq!(ref_tree.children[Side::Left as usize].as_ref().unwrap().value, TEST_1);
        assert_eq!(ref_tree.children[Side::Left as usize].as_ref().unwrap().children[Side::Left as usize].as_ref().unwrap().value, TEST_2);
        assert_eq!(ref_tree.children[Side::Left as usize].as_ref().unwrap().children[Side::Right as usize].as_ref().unwrap().value, TEST_4);
        assert_eq!(ref_tree.children[Side::Right as usize].as_ref().unwrap().value, TEST_5);
    }

    #[test]
    fn test_rotate_left_right() {
        let mut tree: Option<Box<Node<u64>>> = Some(Box::new(Node {
            children: [
                Some(Box::new(Node {
                    children: [None, None],
                    value: TEST_2,
                    height: 1,
                })),
                Some(Box::new(Node {
                    children: [Some(Box::new(Node {
                        children: [None, None],
                        value: TEST_4,
                        height: 1,
                    })), Some(Box::new(Node {
                        children: [None, None],
                        value: TEST_5,
                        height: 1,
                    }))],
                    value: TEST_3,
                    height: 2,
                })),
            ],
            value: TEST_1,
            height: 3,
        }));

        let res: bool = tree.as_mut().unwrap().rotate(Side::Left);
        assert!(res);
        let res: bool = tree.as_mut().unwrap().rotate(Side::Right);
        assert!(res);
        let ref_tree: &Box<Node<u64>> = tree.as_ref().unwrap();
        assert_eq!(ref_tree.value, TEST_1);
        assert_eq!(ref_tree.children[Side::Left as usize].as_ref().unwrap().value, TEST_2);
        assert_eq!(ref_tree.children[Side::Right as usize].as_ref().unwrap().value, TEST_3);
        assert_eq!(ref_tree.children[Side::Right as usize].as_ref().unwrap().children[Side::Left as usize].as_ref().unwrap().value, TEST_4);
        assert_eq!(ref_tree.children[Side::Right as usize].as_ref().unwrap().children[Side::Right as usize].as_ref().unwrap().value, TEST_5);
    }

    #[test]
    fn test_rotate_right_left() {
        let mut tree: Option<Box<Node<u64>>> = Some(Box::new(Node {
            children: [
                Some(Box::new(Node {
                    children: [Some(Box::new(Node {
                        children: [None, None],
                        value: TEST_4,
                        height: 1,
                    })), Some(Box::new(Node {
                        children: [None, None],
                        value: TEST_5,
                        height: 1,
                    }))],
                    value: TEST_2,
                    height: 2,
                })),
                Some(Box::new(Node {
                    children: [None, None],
                    value: TEST_3,
                    height: 1,
                })),
            ],
            value: TEST_1,
            height: 3,
        }));
        let res: bool = tree.as_mut().unwrap().rotate(Side::Right);
        assert!(res);
        let res: bool = tree.as_mut().unwrap().rotate(Side::Left);
        assert!(res);
        let ref_tree: &Box<Node<u64>> = tree.as_ref().unwrap();
        assert_eq!(ref_tree.value, TEST_1);
        assert_eq!(ref_tree.children[Side::Left as usize].as_ref().unwrap().value, TEST_2);
        assert_eq!(ref_tree.children[Side::Left as usize].as_ref().unwrap().children[Side::Left as usize].as_ref().unwrap().value, TEST_4);
        assert_eq!(ref_tree.children[Side::Left as usize].as_ref().unwrap().children[Side::Right as usize].as_ref().unwrap().value, TEST_5);
        assert_eq!(ref_tree.children[Side::Right as usize].as_ref().unwrap().value, TEST_3);
    }

    #[test]
    fn test_rebalance() {
        let mut tree: Option<Box<Node<u64>>> = Some(Box::new(Node {
            children: [
                Some(Box::new(Node {
                    children: [Some(Box::new(Node {
                        children: [None, None],
                        value: TEST_3,
                        height: 1,
                    })), None],
                    value: TEST_2,
                    height: 2,
                })),
                None,
            ],
            value: TEST_1,
            height: 3,
        }));
        let ref_tree: Option<&Box<Node<u64>>> = tree.as_ref();
        assert!(ref_tree.is_some());
        assert_eq!(ref_tree.unwrap().sanity_check(), true);
        assert_eq!(ref_tree.unwrap().is_balanced(), false);
        assert_eq!(ref_tree.unwrap().depth(), ref_tree.unwrap().height);
        let res: bool = tree.as_mut().unwrap().rebalance();
        assert!(res);
        let ref_tree: &Box<Node<u64>> = tree.as_ref().unwrap();
        assert_eq!(ref_tree.sanity_check(), true);
        assert_eq!(ref_tree.is_balanced(), true);
        assert_eq!(ref_tree.depth(), ref_tree.height);
    }

    #[test]
    fn test_rebalance_twice() {
        let mut tree: Option<Box<Node<u64>>> = Some(Box::new(Node {
            children: [
                Some(Box::new(Node {
                    children: [Some(Box::new(Node {
                        children: [None, None],
                        value: TEST_3,
                        height: 1,
                    })), None],
                    value: TEST_2,
                    height: 2,
                })),
                None,
            ],
            value: TEST_1,
            height: 3,
        }));
        let res: bool = tree.as_mut().unwrap().rebalance();
        assert!(res);
        let res: bool = tree.as_mut().unwrap().rebalance();
        assert!(!res);
    }

    #[test]
    fn test_create_tree() {
        let tree: Tree<u64> = Node::create_tree(&TEST_1);
        let ref_tree: Option<&Box<Node<u64>>> = tree.as_ref();
        assert!(ref_tree.is_some());
        assert_eq!(ref_tree.unwrap().value, TEST_1);
        assert_eq!(ref_tree.unwrap().height, 1);
        assert!(ref_tree.unwrap().children[Side::Left as usize].is_none());
        assert!(ref_tree.unwrap().children[Side::Right as usize].is_none());
    }

    #[test]
    fn test_create_node() {
        let node: Node<u64> = Node::create_node(&TEST_1);
        assert_eq!(node.value, TEST_1);
        assert_eq!(node.height, 1);
        assert!(node.children[Side::Left as usize].is_none());
        assert!(node.children[Side::Right as usize].is_none());
    }

    #[test]
    fn test_insert() {
        let mut node: Node<u64> = Node::create_node(&TEST_1);
        let res: bool = node.insert(TEST_2);
        assert!(res);
        let res: bool = node.insert(TEST_3);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.height, 4)
    }

    #[test]
    fn test_width_depth_count() {
        let mut node: Node<u64> = Node::create_node(&TEST_4);
        assert_eq!(node.count(), 1);
        assert_eq!(node.depth(), 1);
        assert_eq!(node.width(), 1);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 2);
        assert_eq!(node.depth(), 2);
        assert_eq!(node.width(), 2);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 3);
        assert_eq!(node.depth(), 2);
        assert_eq!(node.width(), 2);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 4);
        assert_eq!(node.depth(), 3);
        assert_eq!(node.width(), 3);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 5);
        assert_eq!(node.depth(), 3);
        assert_eq!(node.width(), 3);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 6);
        assert_eq!(node.depth(), 3);
        assert_eq!(node.width(), 4);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 7);
        assert_eq!(node.depth(), 3);
        assert_eq!(node.width(), 4);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 8);
        assert_eq!(node.depth(), 4);
        assert_eq!(node.width(), 5);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 9);
        assert_eq!(node.depth(), 4);
        assert_eq!(node.width(), 5);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 10);
        assert_eq!(node.depth(), 4);
        assert_eq!(node.width(), 6);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 11);
        assert_eq!(node.depth(), 4);
        assert_eq!(node.width(), 6);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 12);
        assert_eq!(node.depth(), 4);
        assert_eq!(node.width(), 7);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 13);
        assert_eq!(node.depth(), 4);
        assert_eq!(node.width(), 7);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 14);
        assert_eq!(node.depth(), 4);
        assert_eq!(node.width(), 8);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 15);
        assert_eq!(node.depth(), 4);
        assert_eq!(node.width(), 8);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.count(), 16);
        assert_eq!(node.depth(), 5);
        assert_eq!(node.width(), 9);
    }

    #[test]
    fn test_delete() {
        let mut tree: Tree<u64> = Node::create_tree(&TEST_1);
        let node: &mut Box<Node<u64>> = tree.as_mut().unwrap();
        let res: bool = node.insert(TEST_2);
        assert!(res);
        let res: bool = node.insert(TEST_3);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.height, 4);
        Node::delete(&mut tree);
    }

    #[test]
    fn test_min_max() {
        let mut tree: Tree<u64> = Node::create_tree(&TEST_1);
        let node: &mut Box<Node<u64>> = tree.as_mut().unwrap();
        let res: bool = node.insert(TEST_2);
        assert!(res);
        let res: bool = node.insert(TEST_3);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(node.max(), &TEST_2);
        assert_eq!(node.min(), &TEST_1);
    }

    #[test]
    fn test_dump() {
        #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
        struct Position {
            x: i32,
        }
        impl fmt::Display for Position {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "\"{}\"", self.x)
            }
        }
        let mut tree: Tree<Position> = Node::create_tree(&Position {
            x: 10
        });
        let node: &mut Box<Node<Position>> = tree.as_mut().unwrap();

        assert_eq!(node.dump(false), "[\"10\",null,null]");
        let res: bool = node.insert(Position {
            x: 20
        });
        assert!(res);
        let res: bool = node.insert(Position {
            x: 30
        });
        assert!(res);
        let res: bool = node.insert(Position {
            x: 50
        });
        assert!(res);
        let res: bool = node.insert(Position {
            x: 40
        });
        assert!(res);

        assert_eq!(node.dump(false), "[\"20\",[\"10\",null,null],[\"40\",[\"30\",null,null],[\"50\",null,null]]]");
        assert_eq!(node.dump(true), r#"[
   "20",
   [
      "10",
      null,
      null
   ],
   [
      "40",
      [
         "30",
         null,
         null
      ],
      [
         "50",
         null,
         null
      ]
   ]
]"#)
    }

    #[test]
    fn test_remove_min() {
        let mut tree: Tree<u64> = Node::create_tree(&TEST_1);

        let node: &mut Box<Node<u64>> = tree.as_mut().unwrap();
        let res: bool = node.insert(TEST_2);
        assert!(res);
        let res: bool = node.insert(TEST_3);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(tree.as_ref().unwrap().height, 3);
        assert!(tree.as_ref().unwrap().is_balanced());
        let res: u64 = Node::remove_min(&mut tree);
        assert_eq!(res, TEST_1);
        assert_eq!(tree.as_ref().unwrap().height, 2);
        assert!(tree.as_ref().unwrap().is_balanced());
    }

    #[test]
    fn test_remove() {
        let mut tree: Tree<u64> = Node::create_tree(&TEST_1);

        let node: &mut Box<Node<u64>> = tree.as_mut().unwrap();
        let res: bool = node.insert(TEST_2);
        assert!(res);
        let res: bool = node.insert(TEST_3);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(tree.as_ref().unwrap().height, 3);
        assert!(tree.as_ref().unwrap().is_balanced());
        let res: Option<u64> = Node::remove(&mut tree, &TEST_4);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), TEST_4);
        assert_eq!(tree.as_ref().unwrap().height, 2);
        assert!(tree.as_ref().unwrap().is_balanced());
    }

    #[test]
    fn test_remove_complex() {
        let mut tree: Tree<u64> = Node::create_tree(&TEST_1);

        let node: &mut Box<Node<u64>> = tree.as_mut().unwrap();
        let res: bool = node.insert(TEST_2);
        assert!(res);
        let res: bool = node.insert(TEST_3);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        assert_eq!(tree.as_ref().unwrap().height, 3);
        assert!(tree.as_ref().unwrap().is_balanced());
        let res: Option<u64> = Node::remove(&mut tree, &TEST_4);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), TEST_4);
        assert_eq!(tree.as_ref().unwrap().height, 2);
        assert!(tree.as_ref().unwrap().is_balanced());

        let res: Option<u64> = Node::remove(&mut tree, &TEST_4);
        assert!(res.is_none());

        let res: Option<u64> = Node::remove(&mut tree, &TEST_2);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), TEST_2);

        let res: Option<u64> = Node::remove(&mut tree, &TEST_1);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), TEST_1);

        let res: Option<u64> = Node::remove(&mut tree, &TEST_3);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), TEST_3);

        assert!(tree.as_ref() == None)
    }


    #[test]
    fn test_get() {
        let mut tree: Tree<u64> = Node::create_tree(&TEST_1);
        let node: &mut Box<Node<u64>> = tree.as_mut().unwrap();
        let res: bool = node.insert(TEST_2);
        assert!(res);
        let res: bool = node.insert(TEST_3);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        let tree: &Tree<u64> = Node::get(&tree, &TEST_2);
        assert!(tree.is_some());
        assert_eq!(tree.as_ref().unwrap().value, TEST_2);
        assert_eq!(tree.as_ref().unwrap().height, 2);
    }

    #[test]
    fn test_get_missing() {
        let mut tree: Tree<u64> = Node::create_tree(&TEST_1);
        let node: &mut Box<Node<u64>> = tree.as_mut().unwrap();
        let res: bool = node.insert(TEST_2);
        assert!(res);
        let res: bool = node.insert(TEST_3);
        assert!(res);
        let res: bool = node.insert(TEST_4);
        assert!(res);
        let tree: &Tree<u64> = Node::get(&tree, &TEST_5);
        assert!(tree.is_none());
    }

    #[test]
    fn test_get_remove() {
        let mut tree: Tree<u64> = Node::create_tree(&TEST_1);
        let node: &mut Box<Node<u64>> = tree.as_mut().unwrap();
        let res: bool = node.insert(TEST_2);
        assert!(res);
        let res: bool = node.insert(TEST_3);
        assert!(res);
        let tree2: &Tree<u64> = Node::get(&tree, &TEST_2);
        assert!(tree2.is_some());
        assert_eq!(tree2.as_ref().unwrap().value, TEST_2);
        let old: u64 = tree2.as_ref().unwrap().value.clone();
        let removed: Option<u64> = Node::remove(&mut tree, &TEST_2);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap(), TEST_2);
        assert_eq!(old, TEST_2);
        let removed: Option<u64> = Node::remove(&mut tree, &TEST_2);
        assert!(removed.is_none());
        let tree2: &Tree<u64> = Node::get(&tree, &TEST_2);
        assert!(tree2.is_none());
    }

    #[test]
    fn test_get_not_modifying() {
        let mut tree: Tree<u64> = Node::create_tree(&TEST_1);
        let node: &mut Box<Node<u64>> = tree.as_mut().unwrap();
        let res: bool = node.insert(TEST_2);
        assert!(res);
        let res: bool = node.insert(TEST_3);
        assert!(res);
        let tree_get: &Tree<u64> = Node::get(&tree, &TEST_3);
        assert!(tree_get.is_some());
        assert_eq!(tree_get.as_ref().unwrap().value, TEST_3);
        let mut tree2: Tree<u64> = tree_get.clone();
        let removed: Option<u64> = Node::remove(&mut tree, &TEST_2);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap(), TEST_2);
        let removed2: Option<u64> = Node::remove(&mut tree2, &TEST_2);
        assert!(removed2.is_some());
        assert_eq!(removed2.unwrap(), TEST_2);
        assert_eq!(tree2, tree);
        let l_ptr: String = format!("{:018p}", tree.unwrap());
        let r_ptr: String = format!("{:018p}", tree2.unwrap());
        assert_ne!(l_ptr, r_ptr);
    }
}

impl<'a, T: 'a + Clone + Ord + Eq + Debug + Display> Node<T> {
    fn insert(&mut self, new_payload: T) -> bool {
        if self.value == new_payload {
            // we could decide to not insert double nodes
            //return false ;
        }
        let target_node: &mut Tree<T> = if new_payload <= self.value { &mut self.children[Side::Left as usize] } else { &mut self.children[Side::Right as usize] };
        match target_node {
            &mut Some(ref mut subnode) => {
                subnode.insert(new_payload);
            }
            &mut None => {
                let new_node = Node { children: [None, None], value: new_payload, height: 1 };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
        self.update_height();
        self.rebalance();
        true
    }

    fn create_tree(value: &T) -> Tree<T> {
        Some(Box::new(Self::create_node(value)))
    }

    fn create_node(value: &T) -> Self {
        Node {
            children: [None, None],
            value: value.clone(),
            height: 1,
        }
    }

    fn delete(node: &mut Tree<T>) {
        if node.is_some() {
            Self::delete(node.as_mut().unwrap().children[Side::Left as usize].take().borrow_mut());
            Self::delete(node.as_mut().unwrap().children[Side::Right as usize].take().borrow_mut());
            node.take();
        }
    }

    fn remove_min(node: &mut Tree<T>) -> T {
        if node.is_none() {
            panic!("You should not pass a NULL in that function");
        }
        if node.as_ref().unwrap().children[Side::Left as usize].is_none() {
            let right = node.as_mut().unwrap().children[Side::Right as usize].take();
            return replace(node, right).unwrap().value;
        }
        let value: T = Self::remove_min(&mut node.as_mut().unwrap().children[Side::Left as usize]);
        node.as_mut().unwrap().update_height();
        node.as_mut().unwrap().rebalance();
        return value;
    }

    fn remove(node: &mut Tree<T>, value: &T) -> Option<T> {
        if node.is_some() {
            return if &node.as_ref().unwrap().value == value {
                if node.as_ref().unwrap().children[Side::Right as usize].is_some() {
                    let value: T = Self::remove_min(&mut node.as_mut().unwrap().children[Side::Right as usize]);
                    let old: T = replace(&mut node.as_mut().unwrap().value, value);
                    node.as_mut().unwrap().rebalance();
                    Some(old)
                } else {
                    let left: Tree<T> = node.as_mut().unwrap().children[Side::Left as usize].take();
                    let tree: Tree<T> = replace(node, left);
                    if node.is_some() {
                        node.as_mut().unwrap().rebalance();
                    }
                    tree.map_or(None, |node| Some(node.value))
                }
            } else {
                let target_node: &mut Tree<T> = if value <= &node.as_mut().unwrap().value { &mut node.as_mut().unwrap().children[Side::Left as usize] } else { &mut node.as_mut().unwrap().children[Side::Right as usize] };
                let res: Option<T> = Self::remove(target_node, value);
                node.as_mut().unwrap().rebalance();
                res
            };
        }
        None
    }

    fn get(tree: &'a Tree<T>, value: &T) -> &'a Tree<T> {
        if tree.is_some() {
            if &tree.as_ref().unwrap().value <= value && &tree.as_ref().unwrap().value >= value {
                return tree;
            }
            return if value > &tree.as_ref().unwrap().value {
                Node::get(&tree.as_ref().unwrap().children[Side::Right as usize], value)
            } else {
                Node::get(&tree.as_ref().unwrap().children[Side::Left as usize], value)
            };
        }
        &None
    }

    fn left_height(&self) -> usize {
        self.children[Side::Left as usize].as_ref().map_or(0, |left| left.height)
    }

    fn right_height(&self) -> usize {
        self.children[Side::Right as usize].as_ref().map_or(0, |right| right.height)
    }

    fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height());
    }

    fn height(&self) -> usize {
        self.height
    }

    fn dump(&self, prettify: bool) -> String {
        self._dump(prettify,1)
    }

    fn _dump(&self, prettify: bool, height: usize)->String {
        let mut pad = String::new();
        if prettify {
            pad = "   ".repeat(height);
        }
        let mut string_node: String = format!("[{}{},{}", (if prettify { "\n".to_string() } else { String::new() }) + &*pad, self.value, (if prettify { "\n".to_string() } else { String::new() }) + &*pad);
        if self.children[Side::Left as usize].is_some() {
            string_node += &*self.children[Side::Left as usize].as_ref().unwrap()._dump(prettify, height+1);
        } else {
            string_node += "null";
        }

        if self.children[Side::Right as usize].is_some() {
            string_node += &*(",".to_owned()+&( if prettify { "\n".to_string() + &*pad } else { String::new() }));
            string_node += &*self.children[Side::Right as usize].as_ref().unwrap()._dump(prettify, height+1);
        } else {
            string_node+=",";
            string_node += &*((if prettify { "\n".to_string() } else { String::new() }) + &*pad + "null");
        }
        return string_node + &*(if prettify { "\n".to_string() + &*( "   ".repeat(max(height,1)-1)) } else { String::new() }) + "]";
    }

    /*
     Check only if the height were correctly put in the tree (does not check if balanced
     */
    fn sanity_check(&self) -> bool {
        let mut is_correct: bool = self.height() == (1 + max(self.left_height(), self.right_height()));
        if self.children[Side::Left as usize].is_some() {
            is_correct &= self.children[Side::Left as usize].as_ref().unwrap().sanity_check();
        }
        if self.children[Side::Right as usize].is_some() {
            is_correct &= self.children[Side::Right as usize].as_ref().unwrap().sanity_check();
        }
        return is_correct;
    }

    fn is_balanced(&self) -> bool {
        let mut lh: usize = 0;
        let mut lb: bool = true;
        if self.children[Side::Left as usize].is_some() {
            lh = self.children[Side::Left as usize].as_ref().unwrap().depth();
            lb = self.children[Side::Left as usize].as_ref().unwrap().is_balanced();
        }
        let mut rh: usize = 0;
        let mut rb: bool = true;
        if self.children[Side::Right as usize].is_some() {
            rh = self.children[Side::Right as usize].as_ref().unwrap().depth();
            rb = self.children[Side::Right as usize].as_ref().unwrap().is_balanced();
        }
        let diff: usize;
        if lh < rh {
            diff = rh - lh;
        } else {
            diff = lh - rh;
        }
        if diff <= 1 && lb && rb {
            return true;
        }
        return false;
    }

    fn depth(&self) -> usize {
        let mut left: usize = 0;
        if self.children[Side::Left as usize].is_some() {
            left = self.children[Side::Left as usize].as_ref().unwrap().depth()
        }
        let mut right: usize = 0;
        if self.children[Side::Right as usize].is_some() {
            right = self.children[Side::Right as usize].as_ref().unwrap().depth()
        }
        return 1 + max(left, right);
    }

    fn balance_factor(&self) -> i8 {
        let left_height = self.left_height();
        let right_height = self.right_height();

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }

    fn rotate(&mut self, side: Side) -> bool {
        if self.children[!side as usize].is_none() {
            return false;
        }

        let right_node: &mut Box<Node<T>> = self.children[!side as usize].as_mut().unwrap();
        let right_left_tree = right_node.children[side as usize].take();
        let right_right_tree = right_node.children[!side as usize].take();

        let mut new_left_tree = replace(&mut self.children[!side as usize], right_right_tree);
        swap(&mut self.value, &mut new_left_tree.as_mut().unwrap().value);
        let left_tree = self.children[side as usize].take();

        let new_left_node = new_left_tree.as_mut().unwrap();
        new_left_node.children[!side as usize] = right_left_tree;
        new_left_node.children[side as usize] = left_tree;
        self.children[side as usize] = new_left_tree;

        if let Some(node) = self.children[side as usize].as_mut() {
            node.update_height();
        }

        self.update_height();

        true
    }

    fn rebalance(&mut self) -> bool {
        match self.balance_factor() {
            -2 => {
                let right_node = self.children[Side::Right as usize].as_mut().unwrap();
                if right_node.balance_factor() == 1 {
                    right_node.rotate(Side::Right);
                }
                self.rotate(Side::Left);
                true
            }
            2 => {
                let left_node = self.children[Side::Left as usize].as_mut().unwrap();
                if left_node.balance_factor() == -1 {
                    left_node.rotate(Side::Left);
                }
                self.rotate(Side::Right);
                true
            }
            _ => {
                self.update_height();
                false
            }
        }
    }

    fn count(&self) -> usize {
        let left: usize;
        if self.children[Side::Left as usize].is_some() {
            left = self.children[Side::Left as usize].as_ref().unwrap().count()
        } else {
            left = 0;
        }
        let right: usize;
        if self.children[Side::Right as usize].is_some() {
            right = self.children[Side::Right as usize].as_ref().unwrap().count()
        } else {
            right = 0;
        }
        return 1 + left + right;
    }

    fn max(&self) -> &T {
        if self.children[Side::Right as usize].is_some() {
            self.children[Side::Right as usize].as_ref().unwrap().max()
        } else {
            return &self.value;
        }
    }

    fn min(&self) -> &T {
        if self.children[Side::Left as usize].is_some() {
            self.children[Side::Left as usize].as_ref().unwrap().min()
        } else {
            return &self.value;
        }
    }

    fn width(&self) -> usize {
        let left: usize;
        if self.children[Side::Left as usize].is_some() {
            left = self.children[Side::Left as usize].as_ref().unwrap().width()
        } else {
            left = 1;
        }
        let right: usize;
        if self.children[Side::Right as usize].is_some() {
            right = self.children[Side::Right as usize].as_ref().unwrap().width()
        } else {
            right = 1;
        }
        if self.children[Side::Left as usize].is_none() && self.children[Side::Right as usize].is_none() {
            return 1;
        }
        return left + right;
    }
}

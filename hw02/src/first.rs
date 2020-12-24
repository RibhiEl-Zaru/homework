use std::mem;

#[derive(Debug)]
pub struct BST {
    head: Link
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,   
}


impl BST {
    pub fn new() -> Self {
        BST { head: Link::Empty }
    }

    /*
    Insert an element into the BST. Return
    true if successful, or false if the element was already in the BST.
    */
    pub fn insert(&mut self, num: i32) -> bool {
        return self.head.insert(num);
    }

    /*
    Search for an element in the BST. Return
    true iff the element was found.
    */
    pub fn search(&self, num: i32) -> bool{
        return self.head.search(num);
    }   
}


impl Link {
    pub fn insert(&mut self, num: i32)-> bool {
        match self {
            Link::Empty => { 
                *self = Link::More(Box::new(Node{ elem : num, left : Link::Empty, right : Link::Empty}));
                return true;
            }
            Link::More(node) => {
                if num == node.elem {
                    return false;
                }
                else if num > node.elem {
                    return node.right.insert(num);
                }
                else {
                    return node.left.insert(num);
                }
            }
        }
    }

    pub fn search(&self, num: i32) -> bool {
        match self {
            Link::Empty => { 
                return false;
            }
            Link::More(node) => {
                if num == node.elem {
                    return true;
                }
                else if num > node.elem {
                    return node.right.search(num);
                }
                else {
                    return node.left.search(num);
                }
            }
        }
    }

}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn test_BST() {
        // ...
        let mut bst = BST::new();
        assert!(bst.insert(15));
        assert!(bst.insert(2));
        assert!(bst.insert(20));
        
        assert!(bst.search(2));
        assert!(bst.search(20));
        assert!(bst.search(15));
        assert!(!bst.search(2220));

        
        assert!(bst.insert(2220));
        assert!(bst.search(2220));
        
 
        println!("{:#?}", bst);

    }
}
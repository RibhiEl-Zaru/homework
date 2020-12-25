#[derive(Debug)]
pub struct BST<T: Ord> {
    head: Link<T>
}


// yay type aliases!
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T: Ord> {
    elem: T,
    left: Link<T>,
    right: Link<T>,   
}


impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST { head: None }
    }

    /*
    Insert an element into the BST. Return
    true if successful, or false if the element was already in the BST.
    */
    pub fn insert(&mut self, num: T) -> bool {
        return self.head.insert(num);
    }

    /*
    Search for an element in the BST. Return
    true iff the element was found.
    */
    pub fn search(&self, num: T) -> bool{
        return self.head.search(num);
    }   
}

trait InsertSearch<T: Ord>{
    fn insert(&mut self, num: T)-> bool;
    fn search(&self, e: T) -> bool;
}

impl<T:Ord> InsertSearch<T>  for Link<T> {
    fn insert(&mut self, num: T)-> bool {
        match self {
            None => { 
                *self = Some(Box::new(Node{ elem : num, left : None, right : None}));
                return true;
            }
            Some(node) => {
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

    fn search(&self, num: T) -> bool {
        match self {
            None => { 
                return false;
            }
            Some(node) => {
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
// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T: Ord>(BST<T>);

impl<T: Ord> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T: Ord> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        match self.0.head.take() {
            None => {
                return None;
            }
            Some(z) => {
                self.0.head = z.right;
                return Some(z.elem);
            }
        };
    }
}





pub struct Iter<'a, T: Ord> {
    next: Option<&'a Node<T>>,
}


impl<'a, T: Ord> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            next: self.head.as_ref().map(|node| &**node)
        }
    }
}

// We *do* have a lifetime here, because Iter has one that we need to define
impl<'a, T: Ord> Iterator for Iter<'a, T> {
    // Need it here too, this is a type declaration
    type Item = &'a T;

    // None of this needs to change, handled by the above.
    // Self continues to be incredibly hype and amazing
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        match self.next.take() {
            None => {
                return None;
            }
            Some(z) => {
                self.next = z.right.as_ref().map(|node| &**node);
                return Some(&z.elem);
            }
        };
    }
}


pub struct IterMut<'a, T: Ord> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T: Ord> IntoIterator for &'a mut BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            next: self.head.as_ref().map(|node| &**node)
        }
    }
}


impl<'a, T: Ord> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.right.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}



#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn test_bst() {
        // ...
        println!("Running test bst");
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
        
        for elt in bst { // calls bst.into_iter()
            println!("{}", elt);
        }


    }

    #[test]
    fn test_into_iter(){
        println!("Running test iteration");
        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(3);

        println!("Ref based iter");
        for elt in &bst { // calls bst.into_iter()
            println!("{}", elt);
        }

        println!("mutable based iter");
        for elt in &mut bst { // calls bst.into_iter()
            println!("{}", elt);
        }

        println!("Original to iter");
        for elt in bst { // calls bst.into_iter()
            println!("{}", elt);
        }

    }
}
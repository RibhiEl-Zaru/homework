use std::io;
use rand::Rng;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
/// An element of the stack. May be either integer or boolean.
pub enum Elt {
    Int(i32),
    Bool(bool),
}

#[derive(Debug)]
/// An RPN calculator error.
pub enum Error {
    /// Tried to pop from an empty stack.
    Underflow,
    /// Tried to operate on invalid types (e.g. 4 + true)
    Type,
    /// Unable to parse the input.
    Syntax,
    /// Some IO error occurred.
    IO(io::Error),
    /// The user quit the program (with `quit`).
    Quit,
}

#[derive(Debug)]
/// Types of RPN calculator operations.
pub enum Op {
    /// Adds two numbers: pop x, pop y, push x + y.
    Add,
    /// Checks equality of two values: pop x, pop y, push x == y.
    Eq,
    /// Negates a value: pop x, push ~x.
    Neg,
    /// Swaps two values: pop x, pop y, push x, push y.
    Swap,
    /// Computes a random number: pop x, push random number in [0, x).
    Rand,
    /// Quit the calculator.
    Quit,
}


pub struct Stack {
    elements: Vec<Elt>,
    ops: Vec<Op>
}

// Define a generic alias for a `Result` with the return type Elt and error type our custom Error`.
pub type Result<Elt> = std::result::Result<Elt, Error>;


impl Stack {
    /// Creates a new Stack
    pub fn new() -> Stack {
        return Stack{ elements: vec![], ops: vec![] };
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, val: Elt) -> Result<()> {
        return Ok(self.elements.push(val));
    }

    /// Tries to pop a value off of the stack.
    pub fn pop(&mut self) -> Result<Elt> {
        match self.elements.pop() {
            Some(elt) => {
                return Ok(elt);
            }
            None => {
                return Err(Error::Underflow);
            }
        }
    }

    /// Tries to evaluate an operator using values on the stack.
    #[warn(unused_doc_comments)]
    pub fn eval(&mut self, op: Op) -> Result<()> {
        match op {
            // Adds two numbers: pop x, pop y, push x + y.
            Op::Add => {
                match self.pop() {
                    Ok(Elt::Int(el1)) => {
                        match self.pop() {
                            Ok(Elt::Int(el2)) => {
                                return self.push(Elt::Int(el1 + el2));
                            }
                            Ok(Elt::Bool(_))  => { return Err(Error::Type); }
                            Err(error) => {return Err(error);}
                        }
                        
                    }
                    Ok(Elt::Bool(_)) => {
                        return Err(Error::Type);
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            // Checks equality of two values: pop x, pop y, push x == y.
            Op::Eq => {
                match self.pop() {
                    Ok(el1) => {
                        match self.pop() {
                            Ok(el2) => {
                                return self.push(Elt::Bool(el1 == el2));
                            }
                            Err(error) => { return Err(error); }
                        }
                        
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            // Negates a value: pop x, push ~x.
            Op::Neg => {
                match self.pop() {
                    Ok(Elt::Int(el1)) => { return self.push(Elt::Int(- el1)) }
                    Ok(Elt::Bool(bool_res)) => { return self.push(Elt::Bool(!bool_res)) }
                    Err(error) => { return Err(error) }
                }
            }
            // Swaps two values: pop x, pop y, push x, push y.
            Op::Swap => {
                match self.pop() {
                    Ok(x) => {
                        match self.pop() {
                            Ok(y) => { 
                                match self.push(x) {
                                    Ok(_) => {
                                        return self.push(y);
                                    }
                                    Err(err) => { return Err(err); }
                                }
                            }
                            Err(err) => { return Err(err) }
                        }

                    }
                    Err(error) => { return Err(error) }
                }
            }
            // Computes a random number: pop x, push random number in [0, x).
            Op::Rand => {
                
                match self.pop() {
                    Ok(Elt::Int(el1)) => {
                
                        return self.push(Elt::Int(rand::thread_rng().gen_range(0..el1)));
                    }
                    Ok(Elt::Bool(_)) => { return Err(Error::Type)}
                    Err(error) => { return Err(error) }
                }
            }

            // Quit the Calculator
            Op::Quit => {
                return Err(Error::Quit);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_empty1() {
        let mut s = Stack::new();

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_pop_empty2() {
        let mut s = Stack::new();
        s.push(Elt::Int(0)).unwrap();

        let res = s.pop();
        assert!(res.is_ok());

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_add1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Add).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(2));
    }

    #[test]
    fn test_eval_add2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_add3() {
        let mut s = Stack::new();
        s.push(Elt::Bool(true)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_eq1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Eq).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_eq2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_neg1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(-1));
    }

    #[test]
    fn test_eval_neg2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_swap1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        assert!(s.eval(Op::Swap).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(1));
        assert_eq!(s.pop().unwrap(), Elt::Bool(false));

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_swap2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Swap);
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_rand1() {
        let mut s = Stack::new();
        let i = 20;
        s.push(Elt::Int(i)).unwrap();

        assert!(s.eval(Op::Rand).is_ok());

        let rand_val = s.pop().unwrap();
        assert!(rand_val >= Elt::Int(0));
        assert!(rand_val < Elt::Int(i));
    }

    #[test]
    fn test_eval_rand2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Rand);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_quit() {
        let mut s = Stack::new();

        let res = s.eval(Op::Quit);
        assert!(res.is_err());
        if let Err(Error::Quit) = res { } else { assert!(false); }
    }
}
/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    /*
    1) Move m − 1 disks from the source to the spare peg, by the same general solving procedure. Rules are not violated, by assumption. This leaves the disk m as a top disk on the source peg.
    2) Move the disk m from the source to the target peg, which is guaranteed to be a valid move, by the assumptions — a simple step.
    3) Move the m − 1 disks that we have just placed on the spare, from the spare to the target peg by the same general solving procedure, so they are placed on top of the disk m without violating the rules.
    4) The base case being to move 0 disks (in steps 1 and 3), that is, do nothing – which obviously doesn't violate the rules.
    */

    if num_discs > 0 { 
        let mut setup = hanoi(num_discs - 1, src, dst, aux);
        println!("{:?}", setup);
        setup.append(&mut vec![(src, dst)]);
        println!("Move disk from pole {:?} to pole {:?}", src, dst);
        println!("{:?}", setup);
        setup.append(&mut hanoi(num_discs - 1, aux, src, dst));
        println!("{:?}", setup);
        return setup;
    }else {
        return vec![];
    }
    
    
}
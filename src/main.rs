use std::{cell::RefCell, rc::Rc};

use field::Field;

mod field;

pub type Board = Rc<RefCell<Vec<Vec<Field>>>>;

fn main() {
    println!("{}", 9 / 5);
}

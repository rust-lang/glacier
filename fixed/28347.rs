use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut a = 0;
    let fs = vec![Rc::new(RefCell::new(Box::new(|| a = 4 )))];

    fs.iter().map(|f| {
        let mut closure = f.borrow_mut();
        (&mut *closure)()
    });
}

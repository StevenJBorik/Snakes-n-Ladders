//variables by default are immutable, saftey & concurrency 
// Associated functions 'Self' distinct from method usage syntax  
//https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html#associated-functions
 
#[derive(Debug)]
pub struct Board {
    size: i64, 
    start: i64,
    end: i64,
}

impl Board {
    fn board(size: int) -> Self {
        Self {
            start: 0,
            end: start + size - 1,
            size = size, 
        }
    }
}

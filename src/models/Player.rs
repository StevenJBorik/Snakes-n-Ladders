// Constructors/Associated Function syntax - https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html#associated-functions

#[derive(Debug)]
pub struct Player {
    name: string, 
    position: i32,
    won: bool,
}

impl Player {
    fn player(name: string) -> Self {
        Self {
            name: name,
            position: 0,
            won = false,
        }
    }
}


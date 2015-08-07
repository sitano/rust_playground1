use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Clone)]
enum List1<'a, T: 'a> {
    Head(T, &'a List1<'a, T>),
    Tail
}

// 'a explicit lifetime specialization
// + Display - T : must be Displayable
// Rust does not support specialization yet, so no print for Tail yet :(
impl<'a, T: 'a + Display> Display for List1<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use List1::*;

        fn fmt<'a, T: 'a + Display>(l: &List1<'a, T>) -> String {
            match l {
                &Head(ref h, t @ &Head(_, _)) => format!("{}, {}", h, fmt(t)),
                &Head(ref h, &Tail) => format!("{}", h),
                &Tail => "".to_string()
            }
        }

        write!(f, "({})", fmt(self))
    }
}

macro_rules! list1(
    ( $e:expr, $($rest:expr), + ) => ( Head($e, &list1!( $( $rest ), + )) );
    ( $e:expr ) => ( Head($e, &Tail) );
    () => ( Tail::<()> )
);

fn main() {
    use List1::*;

    println!("Print tail: {:?}", Tail::<()>);
    println!("Print some list: {:?}", Head(1, &Head(2, &Head(3, &Tail))));
    println!("Print some lists: {:?}, {:?}", list1!(), list1!(1, 2, 3));
    println!("Nice print a list: {:?}, {}, {}", list1!(), list1!(1), list1!(1, 2, 3));
}
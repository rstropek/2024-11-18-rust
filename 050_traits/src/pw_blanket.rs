mod pointworthy_blanket {
    use crate::{Billable, Pointworthy};

    impl<T: Billable> Pointworthy for T {
        fn points(&self) -> i32 {
            (self.total() / 10.0) as i32
        }
    }
}
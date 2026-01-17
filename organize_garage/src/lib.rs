
#[derive(Debug, PartialEq, Eq)]
pub struct Garage<T> {
    pub left: Option<T>,
    pub right: Option<T>,
}

 impl<T: std::ops::Add<Output = T>> Garage<T>
{
    pub fn move_to_right(&mut self) {
        let l = self.left.take();
        let r = self.right.take();

        self.right = match (r, l) {
            (Some(r), Some(l)) => Some(r + l),
            (Some(r), None) => Some(r),
            (None, Some(l)) => Some(l),
            (None, None) => None,
        };
    }

    pub fn move_to_left(&mut self) {
        let l = self.left.take();
        let r = self.right.take();

        self.left = match (l, r) {
            (Some(l), Some(r)) => Some(l + r),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        };
    }
}

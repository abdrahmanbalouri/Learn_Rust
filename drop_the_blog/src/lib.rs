use std::cell::{Cell, RefCell};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Blog {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Blog {
    pub fn new() -> Self {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_article(&self, body: String) -> (usize, Article<'_>) {
        let id = self.new_id();
        self.states.borrow_mut().push(false);
        (id, Article::new(id, body, self))
    }

    pub fn new_id(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn add_drop(&self, id: usize) {
        self.states.borrow_mut()[id];
        self.drops.set(self.drops.get() + 1);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Article<'a> {
    id: usize,
    body: String,
    parent: &'a Blog,
}

impl<'a> Article<'a> {
    pub fn new(id: usize, body: String, parent: &'a Blog) -> Self {
        Self {
            id: id,
            body: body,
            parent: parent,
        }
    }

    pub fn discard( & self) {
        drop(& mut self)
    }
}

impl Drop for Article<'_> {
    fn drop(& mut  self) {
        self.parent.add_drop(self.id)
    }
}

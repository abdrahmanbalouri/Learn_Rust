#[derive(Debug, PartialEq, Eq)]
pub struct Target {
    pub size: u32,
    pub xp: u32,
}

pub struct Field {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: Target,
    next: Link,
}

impl Field {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, target: Target) {
        let k = Box::new(Node {
            elem: target,
            next: self.head.take(),
        });

        self.head = Some(k);
    }

    pub fn pop(&mut self) -> Option<Target> {
        self.head.take().map(|v| {
            self.head = v.next;
            v.elem
        })
    }

    pub fn peek(&self) -> Option<&Target> {
        self.head.as_ref().map(|c| &c.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut Target> {
        self.head.as_mut().map(|c| &mut c.elem)
    }
}

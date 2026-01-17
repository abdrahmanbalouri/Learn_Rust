#[derive(Debug, Clone)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub discount: i32,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: None }
    }

    pub fn add(&mut self, name: String, discount: i32) {
        let new = Box::new(Person {
            name: name,
            discount: discount,
            next_person: self.node.take(),
        });
        self.node = Some(new);
    }

    pub fn invert_queue(&mut self) {
        let mut prev = None;
        let mut cur = self.node.take();

        while let Some(mut node) = cur {
            let next = node.next_person.take();
            node.next_person = prev;
            prev = Some(node);
            cur = next;
        }
        self.node = prev;
    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        if self.node.is_none() {
            return None;
        }

        if self.node.as_ref().unwrap().next_person.is_none() {
            let k = self.node.take().unwrap();
            return Some((k.name, k.discount));
        }

        let mut cur = self.node.as_mut().unwrap();
        while cur.next_person.as_ref().unwrap().next_person.is_some() {
            cur = cur.next_person.as_mut().unwrap();
        }
        let last = cur.next_person.take().unwrap();
        return Some((last.name, last.discount));
    }

    pub fn search(&self, name: &str) -> Option<(&String, &i32)> {
        let mut f = self.node.as_deref();

        while let Some(node) = f {
            if node.name == name {
                return Some((&node.name, &node.discount));
            }
            f = node.next_person.as_deref();
        }
        None
    }
}

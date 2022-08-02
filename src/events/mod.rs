use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

type Queue<T> = Rc<RefCell<LinkedList<T>>>;

pub struct Subscriber<T: Clone> {
    queue: Queue<T>,
}

pub struct Topic<T: Clone> {
    subscribers: LinkedList<Queue<T>>,
}

impl<T: Clone> Topic<T> {
    pub fn new() -> Topic<T> {
        Topic { subscribers: LinkedList::new() }
    }

    pub fn send_message(&mut self, msg: T) {
        for s in &self.subscribers {
            s.borrow_mut().push_back(msg.clone());
        }
    }
}

impl<T: Clone> Subscriber<T> {
    pub fn new() -> Subscriber<T> {
        Subscriber { queue: Rc::new(RefCell::new(LinkedList::new())) }
    }

    pub fn pop_message(&mut self) -> Option<T> {
        self.queue.borrow_mut().pop_front()
    }

    pub fn follow(&mut self, topic: &mut Topic<T>) {
        topic.subscribers.push_back(self.queue.clone());
    }
}

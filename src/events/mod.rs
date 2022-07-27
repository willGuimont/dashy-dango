use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

use crate::Abort;

type Queue<T> = Arc<Mutex<LinkedList<T>>>;

pub struct Subscriber<T: Clone> {
    queue: Queue<T>,
}

pub struct Topic<T: Clone> {
    subscribers: Vec<Queue<T>>,
}

impl<T: Clone> Topic<T> {
    pub fn new() -> Topic<T> {
        Topic { subscribers: Vec::new() }
    }

    pub fn send_message(&self, msg: T) {
        for s in &self.subscribers {
            let mut sub_queue = s.lock().abort();
            sub_queue.push_back(msg.clone());
        }
    }
}

impl<T: Clone> Subscriber<T> {
    pub fn new() -> Subscriber<T> {
        Subscriber { queue: Arc::new(Mutex::new(LinkedList::new())) }
    }

    pub fn pop_message(&mut self) -> Option<T> {
        self.queue.lock().abort().pop_front()
    }

    pub fn follow(&mut self, topic: &mut Topic<T>) {
        topic.subscribers.push(self.queue.clone());
    }
}

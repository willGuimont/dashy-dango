use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use crate::Abort;

pub struct Subscriber<T: Clone> {
    queue: Receiver<T>,
    sender: Sender<T>,
}

pub struct Topic<T: Clone> {
    subscribers: Vec<Sender<T>>,
}

impl<T: Clone> Topic<T> {
    pub fn new() -> Topic<T> {
        Topic { subscribers: Vec::new() }
    }

    pub fn send_message(&self, msg: T) {
        for s in &self.subscribers {
            s.send(msg.clone()).abort();
        }
    }
}

impl<T: Clone> Subscriber<T> {
    pub fn new() -> Subscriber<T> {
        let (tx, rx): (Sender<T>, Receiver<T>) = mpsc::channel();
        Subscriber { queue: rx, sender: tx }
    }

    pub fn pop_message(&mut self) -> Option<T> {
        self.queue.try_recv().ok()
    }

    pub fn follow(&mut self, topic: &mut Topic<T>) {
        topic.subscribers.push(self.sender.clone())
    }
}


use std::collections::BinaryHeap;

struct EventManager
{
    event_queue: BinaryHeap<i32>,
}

impl EventManager {
    fn new() -> EventManager {
        EventManager {
            event_queue : BinaryHeap::new()
        }
    }
    fn add(&mut self, value: i32) {
        self.event_queue.push(value)
    }
    fn next(&mut self) -> i32 {
        self.event_queue.pop().unwrap()
    }
}

fn main() {

    let mut manager = EventManager::new();

    manager.add(10);
    manager.add(5);
    manager.add(11);
    manager.add(1);

    println!("{}", manager.next());
    println!("{}", manager.next());
    println!("{}", manager.next());
    println!("{}", manager.next());
}

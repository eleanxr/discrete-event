
use core::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Event {
    execution_time: i32,
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.execution_time.cmp(&self.execution_time))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.execution_time.cmp(&self.execution_time)
    }
}

impl Event {
    fn new(execution_time: i32) -> Event {
        Event{ execution_time }
    }
}

struct EventManager {
    event_queue: BinaryHeap<Event>,
}

impl EventManager {
    fn new() -> EventManager {
        EventManager {
            event_queue : BinaryHeap::new()
        }
    }
    fn add(&mut self, event: Event) {
        self.event_queue.push(event)
    }
    fn next(&mut self) -> Option<Event> {
        self.event_queue.pop()
    }
}

fn main() {
    let mut manager = EventManager::new();

    manager.add(Event::new(10));
    manager.add(Event::new(5));
    manager.add(Event::new(11));
    manager.add(Event::new(1));

    println!("{}", manager.next().unwrap().execution_time);
    println!("{}", manager.next().unwrap().execution_time);
    println!("{}", manager.next().unwrap().execution_time);
    println!("{}", manager.next().unwrap().execution_time);
}

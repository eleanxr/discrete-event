
use core::cmp::Ordering;
use std::collections::BinaryHeap;

enum EventDisposition{
    Delete, Reschedule(i32)
}

#[derive(Eq, PartialEq)]
struct Event {
    execution_time: i32,
    action: fn (i32) -> EventDisposition
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
    fn new(execution_time: i32, action: fn (i32) -> EventDisposition) -> Event {
        Event{ execution_time, action }
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

    let f = |t: i32| -> EventDisposition {
        // println!("Action executed at {}", t);
        EventDisposition::Reschedule(t + 10)
    };

    manager.add(Event::new(10, f));
    manager.add(Event::new(5, f));
    manager.add(Event::new(11, f));
    manager.add(Event::new(1, f));

    let mut current_time: i32;
    let mut last_log_time:i32 = 0;
    let log_frequency = 1000;
    while let Some(event) = manager.next() {
        current_time = event.execution_time;
        let time_since_last_log = current_time - last_log_time;
        if time_since_last_log >= log_frequency {
            println!("t = {}", current_time);
            last_log_time = current_time;
        }
        match &(event.action)(current_time) {
            EventDisposition::Reschedule(t) => manager.add(Event::new(t.clone(), event.action)),
            EventDisposition::Delete => ()
        }
    }

    println!("Simulation complete.");
}

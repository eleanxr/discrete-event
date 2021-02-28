use core::cmp::Ordering;
use std::collections::BinaryHeap;

pub enum EventDisposition {
    Delete,
    Reschedule(i32),
}

pub trait EventAction {
    fn execute(&self, execution_time: i32) -> EventDisposition;
}

struct Event {
    execution_time: i32,
    action: Box<dyn EventAction>,
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.execution_time == other.execution_time
    }
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
    fn new(execution_time: i32, action: Box<dyn EventAction>) -> Event {
        Event {
            execution_time,
            action,
        }
    }
}

struct EventManager {
    event_queue: BinaryHeap<Event>,
}

impl EventManager {
    fn new() -> EventManager {
        EventManager {
            event_queue: BinaryHeap::new(),
        }
    }
    fn add(&mut self, event: Event) {
        self.event_queue.push(event)
    }
    fn next(&mut self) -> Option<Event> {
        self.event_queue.pop()
    }
}

// Begin domain specific

struct PrintAndReschedule {}
impl EventAction for PrintAndReschedule {
    fn execute(&self, execution_time: i32) -> EventDisposition {
        println!("Action executed at {}", execution_time);
        if execution_time < 1000 {
            EventDisposition::Reschedule(execution_time + 10)
        } else {
            EventDisposition::Delete
        }
    }
}

struct Mouse {
    position: (f64, f64),
}

impl Mouse {
    fn new() -> Self {
        return Mouse {
            position: (0.0, 0.0),
        };
    }

    fn move_delta(&mut self, delta: (f64, f64)) {
        self.position.0 += delta.0;
        self.position.1 += delta.1;
    }
}

fn main() {
    let mut manager = EventManager::new();

    manager.add(Event::new(10, Box::new(PrintAndReschedule{})));
    manager.add(Event::new(5, Box::new(PrintAndReschedule{})));
    manager.add(Event::new(11, Box::new(PrintAndReschedule{})));
    manager.add(Event::new(2, Box::new(PrintAndReschedule{})));

    let mut current_time: i32;
    let mut last_log_time: i32 = 0;
    let log_frequency = 1000;
    let max_time = 2000;
    while let Some(event) = manager.next() {
        current_time = event.execution_time;
        let time_since_last_log = current_time - last_log_time;
        if time_since_last_log >= log_frequency {
            println!("t = {}", current_time);
            last_log_time = current_time;
        }

        if let EventDisposition::Reschedule(next_time) = event.action.execute(current_time) {
            manager.add(Event::new(next_time.clone(), event.action));
        }
    }
    println!("Simulation complete.");
}

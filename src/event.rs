use core::cmp::Ordering;
use std::collections::BinaryHeap;

pub enum EventDisposition {
    Delete,
    Reschedule(i32),
}

pub trait EventAction {
    fn execute(&self, execution_time: i32) -> EventDisposition;
}

pub struct Event {
    pub execution_time: i32,
    pub action: Box<dyn EventAction>,
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
    pub fn new(execution_time: i32, action: Box<dyn EventAction>) -> Event {
        Event {
            execution_time,
            action,
        }
    }
}

struct EventExecutor {
    log_frequency: i32,
    last_log_time: i32,
    current_time: i32,
}

impl EventExecutor {
    fn new(log_frequency: i32) -> EventExecutor {
        EventExecutor {
            log_frequency: log_frequency,
            last_log_time: 0,
            current_time: 0,
        }
    }

    fn execute(&mut self, event: &Event) -> EventDisposition {
        self.current_time = event.execution_time;
        let time_since_last_log = self.current_time - self.last_log_time;
        if time_since_last_log >= self.log_frequency {
            println!("t = {}", self.current_time);
            self.last_log_time = self.current_time;
        }
        event.action.execute(self.current_time)
    }
}

pub struct EventManager {
    event_queue: BinaryHeap<Event>,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            event_queue: BinaryHeap::new(),
        }
    }
    
    pub fn add(&mut self, event: Event) {
        self.event_queue.push(event)
    }
    
    pub fn next(&mut self) -> Option<Event> {
        self.event_queue.pop()
    }

    pub fn run(&mut self) {
        let mut executor = EventExecutor::new(100);

        let mut current_time: i32 = 0;
        let max_time = 990;
        while current_time < max_time {
            if let Some(event) = self.next() {
                current_time = event.execution_time;
                if let EventDisposition::Reschedule(time) = executor.execute(&event) {
                    self.add(Event::new(time, event.action));
                }
            }
        }
    }
}

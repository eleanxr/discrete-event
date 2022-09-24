use core::cmp::Ordering;
use std::collections::BinaryHeap;

pub enum EventDisposition<T> {
    Delete,
    Reschedule(T),
}

pub trait EventAction<T> {
    fn execute(&self, execution_time: T) -> EventDisposition<T>;
}

pub struct Event<T> {
    pub execution_time: T,
    pub action: Box<dyn EventAction<T>>,
}

impl<T: Ord> Eq for Event<T> {}

impl<T: Ord> PartialEq for Event<T> {
    fn eq(&self, other: &Self) -> bool {
        self.execution_time == other.execution_time
    }
}

impl<T: Ord> PartialOrd for Event<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.execution_time.cmp(&self.execution_time))
    }
}

impl<T: Ord> Ord for Event<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.execution_time.cmp(&self.execution_time)
    }
}

impl<T> Event<T> {
    pub fn new(execution_time: T, action: Box<dyn EventAction<T>>) -> Event<T> {
        Event {
            execution_time,
            action,
        }
    }
}

struct EventExecutor<T> {
    log_frequency: T,
    last_log_time: T,
    current_time: T,
}

impl<T> EventExecutor<T>
where
    T: std::fmt::Display,
    T: std::ops::Sub<Output = T>,
    T: Ord,
    T: Copy
{
    fn new(start_time: T, log_frequency: T) -> EventExecutor<T> {
        EventExecutor {
            log_frequency: log_frequency,
            last_log_time: start_time,
            current_time: start_time,
        }
    }

    fn execute(&mut self, event: &Event<T>) -> EventDisposition<T> {
        self.current_time = event.execution_time;
        let time_since_last_log: T = self.current_time - self.last_log_time;
        if time_since_last_log >= self.log_frequency {
            println!("t = {}", self.current_time);
            self.last_log_time = self.current_time;
        }
        event.action.execute(self.current_time)
    }
}

pub struct EventManager<T: Ord> {
    event_queue: BinaryHeap<Event<T>>,
}

impl<T> EventManager<T>
where
    T: Ord,
    T: std::fmt::Display,
    T: std::ops::Sub<Output = T>,
    T: Copy
{
    pub fn new() -> EventManager<T> {
        EventManager {
            event_queue: BinaryHeap::new(),
        }
    }

    pub fn add(&mut self, event: Event<T>) {
        self.event_queue.push(event)
    }

    pub fn next(&mut self) -> Option<Event<T>> {
        self.event_queue.pop()
    }

    pub fn run(&mut self, start_time: T, max_time: T, log_interval: T) {
        let mut executor: EventExecutor<T> = EventExecutor::<T>::new(start_time, log_interval);

        let mut current_time: T = start_time.clone();
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

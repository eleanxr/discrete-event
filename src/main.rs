use discrete_event::event::{Event, EventAction, EventDisposition, EventManager};

struct PrintAndReschedule {}

impl EventAction<i32> for PrintAndReschedule {
    fn execute(&self, execution_time: i32) -> EventDisposition<i32> {
        println!("Action executed at {}", execution_time);
        if execution_time < 1000 {
            EventDisposition::Reschedule(execution_time + 10)
        } else {
            EventDisposition::Delete
        }
    }
}

fn main() {
    let mut manager = EventManager::new();

    manager.add(Event::new(10, Box::new(PrintAndReschedule {})));
    manager.add(Event::new(5, Box::new(PrintAndReschedule {})));
    manager.add(Event::new(11, Box::new(PrintAndReschedule {})));
    manager.add(Event::new(2, Box::new(PrintAndReschedule {})));

    manager.run(0, 1000, 100);

    println!("Simulation complete.");
}

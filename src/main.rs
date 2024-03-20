use std::collections::HashMap;
mod scheduler;

use scheduler::types::Event;

// #[tokio::main(flavor = "current_thread")]

fn main() {
    let mut event_map: HashMap<u64, Event> = HashMap::new();
    scheduler::create_event(&mut event_map, 123232, "echo hello".to_string());
    let _dummy_event: Event = Event {
        timestamp: 0,
        command: "".to_string(),
    };
    let item = event_map.get(&123232);
    println!("Item {:?}", &item);
}

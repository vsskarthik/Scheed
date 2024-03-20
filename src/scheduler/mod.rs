pub mod types;
use std::collections::HashMap;

use types::Event;

pub fn create_event(event_map: &mut HashMap<u64, Event>, timestamp: u64, command: String) {
    let event = Event { timestamp, command };
    event_map.insert(event.timestamp, event);
}

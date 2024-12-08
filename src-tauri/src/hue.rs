use sse_client::EventSource;

pub fn monitor_events() {
    let event_source = EventSource::new("http://10.0.0.195/eventsource/clip/v2").unwrap();

    event_source.on_message(|message| {
        println!("New message event {:?}", message);
    });

    event_source.add_event_listener("error", |error| {
        println!("Error {:?}", error);
    });
}

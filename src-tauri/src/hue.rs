use eventsource_client as event_client;
use futures::{Stream, TryStreamExt};

const HUE_URL: &str = "https://10.0.0.195/eventstream/clip/v2";
const HUE_HEADER: &str = "hue-application-key";
const HUE_KEY: &str = "k9ayxh7kwhoKodCqpcdh6vqGxZU3Cydg-4CkyFHu";

pub async fn monitor_events() -> Result<(), event_client::Error> {
    let client = event_client::ClientBuilder::for_url(HUE_URL)
        .unwrap()
        .header(HUE_HEADER, HUE_KEY)
        .unwrap()
        .build();

    let mut stream = tail_events(client);

    while let Ok(Some(_)) = stream.try_next().await {}

    Ok(())
}

fn tail_events(client: impl event_client::Client) -> impl Stream<Item = Result<(), ()>> {
    client
        .stream()
        .map_ok(|event| match event {
            event_client::SSE::Connected(connection) => {
                println!("Connected: \nstatus={}", connection.response().status())
            }
            event_client::SSE::Event(ev) => {
                println!("Event: {}\n{}", ev.event_type, ev.data)
            }
            event_client::SSE::Comment(comment) => {
                println!("Comment: \n{}", comment)
            }
        })
        .map_err(|err| eprintln!("Error streaming events: {:?}", err))
}

use discord_rpc_client::{Client, Event};
use rouille::Response;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{env, io::Read, sync::Mutex, thread, time};

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    title: String,
    artist: String,
    album: String,
    image: String,
    playing: bool,
}

fn main() {
    // Create the client
    let mut drpc = Client::new(1190809920443007037);

    // Register event handlers with the corresponding methods
    drpc.on_ready(|_ctx| {
        println!("ready?");
    });

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    let mutex_drpc = Mutex::new(drpc);

    rouille::start_server("127.0.0.1:4000", move |request| {
        let mut body = String::new();
        request.data().unwrap().read_to_string(&mut body).unwrap();
        let status: Status = serde_json::from_str(&body).unwrap();

        let mut drpc = mutex_drpc.lock().unwrap();

        // Set the activity
        drpc.set_activity(|activity| {
            activity
                .details(format!("Listening to: {}", status.title))
                .state(format!("{} - {}", status.artist, status.album))
                .assets(|assets| {
                    assets
                        .large_image(status.image)
                        .small_image(if status.playing { "play" } else { "pause" })
                        .small_text(if status.playing { "playing" } else { "paused" })
                })
        })
        .expect("Failed to set activity");

        Response::text("")
    });
}

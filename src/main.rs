
/*
use std::env;
use futures::StreamExt;
use telegram_bot::*;

// 5613891155:AAEgh3uQZtEJyuDAtzc8UC8fIea0JmqHk0g

#[tokio::main]
async fn main() -> Result<(), Error>{
    let token = env::var("5613891155:AAEgh3uQZtEJyuDAtzc8UC8fIea0JmqHk0g").expect("5613891155:AAEgh3uQZtEJyuDAtzc8UC8fIea0JmqHk0g not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                api.send(message.text_reply(format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                )))
                .await;
            }
        }
    }
    Ok(())
}*/

use std::env;

use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("5613891155:AAEgh3uQZtEJyuDAtzc8UC8fIea0JmqHk0g").expect("5613891155:AAEgh3uQZtEJyuDAtzc8UC8fIea0JmqHk0g not set");
    let api = Api::new(token);

    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter("telegram_bot=trace")
            .finish(),
    )
    .unwrap();

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                api.send(message.text_reply(format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                )))
                .await?;
            }
        }
    }
    Ok(())
}
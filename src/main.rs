use frankenstein::Api;
use frankenstein::TelegramApi;

static TOKEN: &str = "5613891155:AAEgh3uQZtEJyuDAtzc8UC8fIea0JmqHk0g";

fn main() {
    let api = Api::new(TOKEN);

    match api.get_me() {
        Ok(response) => {
            let user = response.result;
            println!(
                "Hello, I'm @{}, https://t.me/{}",
                user.first_name,
                user.username.expect("The bot must have a username.")
            );
        }
        Err(error) => {
            eprintln!("Failed to get me: {error:?}");
        }
    }
}
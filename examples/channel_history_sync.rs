#[allow(unused_imports)]
use slack_api::sync as slack;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // You can generate a legacy token to quickly test these apis
    // https://api.slack.com/custom-integrations/legacy-tokens
    let token = env::var("SLACK_API_TOKEN").map_err(|_| "SLACK_API_TOKEN env var must be set")?;
    let client = slack::default_client().map_err(|_| "Could not get default_client")?;

    let response = slack::channels::history(
        &client,
        &token,
        &slack::channels::HistoryRequest {
            channel: &env::args()
                .nth(1)
                .ok_or("must specify channel id as argument e.g. C09123456")?,
            ..slack::channels::HistoryRequest::default()
        },
    );

    if let Ok(response) = response {
        if let Some(messages) = response.messages {
            for message in &messages {
                println!("{:?}", message);
            }
            println!("Got {} messages:", messages.len());
        }
    } else {
        println!("{:?}", response);
    }
    Ok(())
}

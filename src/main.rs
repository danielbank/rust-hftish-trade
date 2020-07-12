use alpaca_finance::{Account, Alpaca, StreamMessage, Streamer};
use chrono::{DateTime, Utc};
use futures::{future, StreamExt};
use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "tick_taker",
    about = "An example of an HFT-ish trading algorithm using Alpaca Trading API and Rust"
)]
struct Opt {
    /// Stock to trade
    #[structopt(short, long, default_value = "TSLA")]
    symbol: String,

    /// Maximum number of shares to hold at once. Note that this does not account for any existing position; the algorithm only tracks what is bought as part of its execution.
    #[structopt(short, long, default_value = "500")]
    quantity: u32,

    /// Whether to use the Live Trading API. (Defaults to false which uses the Paper API)
    #[structopt(long)]
    isLive: Option<bool>,

    /// API Key ID. (Can also be set via the APCA_API_KEY_ID environment variable.)
    #[structopt(long)]
    id: Option<String>,

    /// API key secret. (Can also be set via the APCA_API_SECRET_KEY environment variable.)
    #[structopt(long)]
    secret: Option<String>,
}

/// We use Quote objects to represent the bid/ask spread. When we encounter a
/// 'level change', a move of exactly 1 penny, we may attempt to make one
/// trade. Whether or not the trade is successfully filled, we do not submit
/// another trade until we see another level change.
/// Note: Only moves of 1 penny are considered eligible because larger moves
/// could potentially indicate some newsworthy event for the stock, which this
/// algorithm is not tuned to trade.

struct Quote {
    prev_bid: f64,
    prev_ask: f64,
    prev_spread: f64,
    bid: f64,
    ask: f64,
    bid_size: u32,
    ask_size: u32,
    spread: u32,
    traded: bool,
    level_ct: u32,
    time: DateTime<Utc>,
}

// impl Quote {
//     fn new() -> Quote {
//         Quote {
//             prev_bid: 0.0,
//             prev_ask: 0.0,
//             prev_spread: 0.0,
//             bid: 0.0,
//             ask: 0.0,
//             bid_size: 0,
//             ask_size: 0,
//             spread: 0,
//             traded: true,
//             level_ct: 1,
//             time: Utc::now(),
//         }
//     }

//     fn reset(&mut self) {
//         self.traded = false;
//         self.level_ct += 1;
//     }

//     fn update(&mut self, data: Quote) {
//         // Update bid and ask sizes and timestamp
//         self.bid_size = data.bid_size;
//         self.ask_size = data.ask_size;

//         // Check if there has been a level change
//         if self.bid != data.bid_price
//             && self.ask != data.ask_price
//             && (100.0 * (data.ask_price - data.bid_price)).round() / 100.0 == 0.01
//         {
//             // Update bids and asks and time of level change
//             self.prev_bid = self.bid;
//             self.prev_ask = self.ask;
//             self.bid = data.bid_price;
//             self.ask = data.ask_price;
//             self.time = data.timestamp;

//             // Update spreads
//             self.prev_spread = (1000.0)
//         }
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    assert!(opt.quantity > 100, "Quantity must be greater than 100.");

    #[allow(non_snake_case)]
    let APCA_API_KEY_ID = match opt.id {
        Some(id) => id,
        None => {
            env::var("APCA_API_KEY_ID").expect("Missing the APCA_API_KEY_ID environment variable.")
        }
    };
    #[allow(non_snake_case)]
    let APCA_API_SECRET_KEY = match opt.secret {
        Some(secret) => secret,
        None => env::var("APCA_API_SECRET_KEY")
            .expect("Missing the APCA_API_SECRET_KEY environment variable."),
    };

    // Get a connection to the Alpaca Trading API
    let alpaca = match opt.isLive {
        Some(true) => Alpaca::live(&APCA_API_KEY_ID, &APCA_API_SECRET_KEY)
            .await
            .expect("Unable to connect to the Alpaca Paper API."),
        _ => Alpaca::paper(&APCA_API_KEY_ID, &APCA_API_SECRET_KEY)
            .await
            .expect("Unable to connect to the Alpaca Live API."),
    };

    let account = Account::get(&alpaca).await.unwrap();

    println!("I have ${:.2} in my account.", account.cash);

    let streamer = Streamer::new(&alpaca);
    streamer
        .start()
        .await
        .for_each(|msg| {
            match msg {
                StreamMessage::Account(_) => println!("Got an account update!"),
                StreamMessage::Order(_) => println!("Got an order update!"),
                _ => println!("Got an unexpected msg"),
            }
            future::ready(())
        })
        .await;

    Ok(())
}

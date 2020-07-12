# rust-hftish-trade

Trying to implement the [Example HFT-ish Algorithm for Alpaca Trading API](https://github.com/alpacahq/example-hftish) in Rust.

## ⚠️ Experimental Code ⚠️

Please don't quit your day job to become an algo trader using this example code.

## Usage

### Create Alpaca Paper API Key

Alpaca Paper is the sandbox for the Alpaca Trading API. It's like playing with monopoly money. You can play without the code without worrying about losing your hard-earned money.

1. [Sign up for Alpaca](https://app.alpaca.markets/signup)

2. Go to the [Paper Trading Dashboard](https://app.alpaca.markets/paper/dashboard/overview) and generate an API Key via the `Your API Keys` card on the right of the page.

### Run the Example HFT-ish Trading Code

```
ALPACA_ID=<API_ID> ALPACA_KEY=<API_SECRET> cargo run
```

### PARAMETERS

The parameters are following.

--symbol: the stock to trade (defaults to "SNAP")
--quantity: the maximum number of shares to hold at once. Note that this does not account for any existing position; the algorithm only tracks what is bought as part of its execution. (Default 500, minimum 100.)
--key-id: your API key ID. (Can also be set via the APCA_API_KEY_ID environment variable.)
--secret-key: your API key secret. (Can also be set via the APCA_API_SECRET_KEY environment variable.)
--base-url: the URL to connect to. (Can also be set via the APCA_API_BASE_URL environment variable. Defaults to "https://paper-api.alpaca.markets" if using a paper account key, "https://api.alpaca.markets" otherwise.)

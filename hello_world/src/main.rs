use serde::Deserialize;
use std::{fs::File, io::Write, thread, time::Duration};

/// Define a Pricing trait for fetching and saving price data
trait Pricing {
    fn fetch_price(&self) -> Option<f64>;
    fn save_to_file(&self, price: f64);
    fn display_price(&self, price: f64);
}

/// Struct for Bitcoin pricing data
struct Bitcoin;

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Option<f64> {
        let url = "https://api.coindesk.com/v1/bpi/currentprice/BTC.json";
        if let Ok(response) = ureq::get(url).call() {
            let body = response.into_string().unwrap_or_else(|_| "Failed to read body".to_string());
            if let Ok(json) = serde_json::from_str::<BitcoinResponse>(&body) {
                return Some(json.bpi.usd.rate_float);
            } else {
                println!("Failed to deserialize Bitcoin response.");
            }
        } else {
            println!("Failed to fetch Bitcoin data.");
        }
        None
    }

    fn save_to_file(&self, price: f64) {
        let mut file = File::create("bitcoin_price.txt").expect("Failed to create file");
        writeln!(file, "Bitcoin price: ${:.2}", price).expect("Failed to write to file");
    }

    fn display_price(&self, price: f64) {
        println!("Bitcoin price: ${:.2}", price);
    }
}

/// Struct for Ethereum pricing data
struct Ethereum;

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Option<f64> {
        let url = "https://api.coinpaprika.com/v1/tickers/eth-ethereum";
        if let Ok(response) = ureq::get(url).call() {
            let body = response.into_string().unwrap_or_else(|_| "Failed to read body".to_string());
            if let Ok(json) = serde_json::from_str::<EthereumResponse>(&body) {
                return Some(json.quotes.usd.price);
            } else {
                println!("Failed to deserialize Ethereum response.");
            }
        }
        None
    }

    fn save_to_file(&self, price: f64) {
        let mut file = File::create("ethereum_price.txt").expect("Failed to create file");
        writeln!(file, "Ethereum price: ${:.2}", price).expect("Failed to write to file");
    }

    fn display_price(&self, price: f64) {
        println!("Ethereum price: ${:.2}", price);
    }
}

/// Struct for S&P 500 pricing data
struct SP500;

impl Pricing for SP500 {
    fn fetch_price(&self) -> Option<f64> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d";
        if let Ok(response) = ureq::get(url).call() {
            let body = response.into_string().unwrap_or_else(|_| "Failed to read body".to_string());
            if let Ok(json) = serde_json::from_str::<SP500Response>(&body) {
                if let Some(item) = json.chart.result.into_iter().next() {
                    return Some(item.meta.regular_market_price);
                }
            } else {
                println!("Failed to deserialize S&P 500 response.");
            }
        }
        None
    }

    fn save_to_file(&self, price: f64) {
        let mut file = File::create("sp500_price.txt").expect("Failed to create file");
        writeln!(file, "S&P 500 price: ${:.2}", price).expect("Failed to write to file");
    }

    fn display_price(&self, price: f64) {
        println!("S&P 500 price: ${:.2}", price);
    }
}

/// Main function
fn main() {
    let assets: Vec<Box<dyn Pricing>> = vec![Box::new(Bitcoin), Box::new(Ethereum), Box::new(SP500)];

    loop {
        for asset in &assets {
            if let Some(price) = asset.fetch_price() {
                asset.display_price(price); // Display price to the terminal
                asset.save_to_file(price); // Save price to a file
            } else {
                eprintln!("Failed to fetch price.");
            }
        }
        println!("--- Waiting for the next update ---");
        thread::sleep(Duration::from_secs(10));
    }
}

/// Struct for Bitcoin API response
#[derive(Deserialize)]
struct BitcoinResponse {
    bpi: Bpi,
}

#[derive(Deserialize)]
struct Bpi {
    #[serde(rename = "USD")]
    usd: Usd,
}

#[derive(Deserialize)]
struct Usd {
    rate_float: f64,
}

/// Struct for Ethereum API response
#[derive(Deserialize)]
struct EthereumResponse {
    quotes: Quotes,
}

#[derive(Deserialize)]
struct Quotes {
    #[serde(rename = "USD")]
    usd: UsdQuote,
}

#[derive(Deserialize)]
struct UsdQuote {
    price: f64,
}

/// Struct for S&P 500 API response
#[derive(Deserialize)]
struct SP500Response {
    chart: Chart,
}

#[derive(Deserialize)]
struct Chart {
    result: Vec<ResultItem>,
}

#[derive(Deserialize)]
struct ResultItem {
    meta: Meta,
}

#[derive(Deserialize)]
struct Meta {
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: f64,
}

struct Bitcoin{
    api_adress:String,
    file_name:String,
}

struct Etherium{
    api_adress:String,
    file_name:String,
}

struct SP500{
    api_adress:String,
    file_name:String,
}

pub trait Pricing {
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self);
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> f32;
    return 32.0
}

fn main() {
    let btc_api = ""
    let btc_txt = "btc_prices.json"
    
    let b = Bitcoin{api_address:btc_api, file_name:btc_txt};
}
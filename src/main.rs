
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use clap::{Arg, App};
use csv::Writer;
use log::{error, info, debug};
use log4rs;

#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;

#[derive(Debug)]
enum GeneralError{
    NoApiKey,
    CSVError(csv::Error),
    IOError(std::io::Error),
    ReqwestError(reqwest::Error),
}

impl std::error::Error for GeneralError {}

impl std::fmt::Display for GeneralError{   
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            GeneralError::NoApiKey =>write!(f, "No API key is set via the .env variable."),
            GeneralError::CSVError(err) =>write!(f, "Error while writing the CSV file {}", err),
            GeneralError::IOError(err) =>write!(f, "Error while flushing the file {}", err),
            GeneralError::ReqwestError(err) =>write!(f, "Error while fetching data {}", err),
        }
    }
}

impl From<reqwest::Error> for GeneralError{
    fn from(err : reqwest::Error) -> GeneralError{
        GeneralError::ReqwestError(err)
    }
}
impl From<std::io::Error> for GeneralError{
    fn from(err : std::io::Error) -> GeneralError{
        GeneralError::IOError(err)
    }
}
impl From<csv::Error> for GeneralError{
    fn from(err : csv::Error) -> GeneralError{
        GeneralError::CSVError(err)
    }
}


#[derive(Serialize,Deserialize,Debug)]
struct Currency{
    name: String,
    symbol: String,
    quote: Quotes,
}

#[derive(Serialize,Deserialize,Debug)]
struct Quote{
    price: f64,
    percent_change_24h: f64,
    percent_change_7d: f64,
}

#[derive(Serialize,Deserialize,Debug)]
struct CMCResponse{ data : HashMap<String,Currency>}

#[derive(Serialize,Deserialize,Debug)]
struct Quotes(HashMap<String, Quote>);

#[derive(Serialize, Deserialize, Debug)]
struct EODResponse {
    code: String,
    close: f64,
}




#[tokio::main]
async fn main()-> Result<(), GeneralError> {

    dotenv().ok();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Ege A. <egeatesalp@gmail.com>")
        .about("My implementation of one_tutorial")
        .arg(Arg::new("currency_list")
            .long("currencies")
            .about("Pass the currencies you want to query")
            .min_values(1)
            .required(true))
        .arg(Arg::new("etfs_list")
            .long("etfs")
            .about("Pass the ETF symbols to fetch prices for")
            .takes_value(true)
            .required(true))
        .get_matches();
        
    let currencies = matches.value_of("currency_list").expect("No currencies were being passed"); 
    let etfs = matches.value_of("etfs_list").expect("No ETF symbol passed"); 
    
    debug!("Querying the following currecies: {:?} and the following eft's: {:?}", currencies, etfs);
        


    let cmc_pro_api_key = dotenv::var("CMC_API_KEY").expect("CMC token not set");
    let eod_api_key = dotenv::var("EOD_API_KEY").expect("EOD token not set");

    if cmc_pro_api_key.is_empty() {
        error!("Empty CMC API KEY provided! Please set one via the .env file!");
        return Err(GeneralError::NoApiKey);
    }

    let mut params = HashMap::new();
    params.insert("symbol", currencies.to_string());
   
    let client = reqwest::Client::new();
    let resp = client
        .get(" https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
        .header("X-CMC_PRO_API_KEY", cmc_pro_api_key)
        .query(&params)
        .send()
        .await?;


    let resp = resp.json::<CMCResponse>().await?;

    let etf = client.get(format!("https://eodhistoricaldata.com/api/real-time/{}?api_token={}&fmt=json", etfs, eod_api_key))
        .send()
        .await?;

    let amundi_etf =etf.json::<EODResponse>().await?;
   
    debug!("Fetched ETF: {}", amundi_etf.close);

    let mut wtr = Writer::from_path("prices.csv")?;
    wtr.write_record(&["Name", "Symbol", "Price","24HourChange", "7DayChange"])?;

    for (symbol, currency) in resp.data.into_iter() {
           wtr.write_record(&[currency.name, symbol.to_owned(), currency.quote.0.get("USD").unwrap().price.to_string(), currency.quote.0.get("USD").unwrap().percent_change_24h.to_string(),  currency.quote.0.get("USD").unwrap().percent_change_7d.to_string()])?;
    }

    wtr.flush()?;

    info!("Queried {} and wrote CSV file", currencies);
    

    Ok(())

}


use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use clap::{Arg, App};
use csv::Writer;

#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;



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
struct Response{ data : HashMap<String,Currency>}

#[derive(Serialize,Deserialize,Debug)]
struct Quotes(HashMap<String, Quote>);




#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {



        let matches = App::new("My Super Program")
            .version("1.0")
            .author("Ege A. <egeatesalp@gmail.com>")
            .about("My implementation of one_tutorial")
            .arg(Arg::new("currency_list")
                .about("Pass the currencies you want to query")
                .min_values(1)
                .required(true))
            .get_matches();
        
        let currencies = matches.value_of("currency_list").expect("No currencies were being passed");  
        
    
    let mut wtr = Writer::from_path("prices.csv")?;
    wtr.write_record(&["Name", "Symbol", "Price","24HourChange", "7DayChange"])?;
    
    let mut params = HashMap::new();


    params.insert("symbol", currencies.to_string());
    dotenv().ok();



    
   
    let client = reqwest::Client::new();
    let resp = client
        .get(" https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
        .header("X-CMC_PRO_API_KEY", dotenv!("API_KEY"))
        .query(&params)
        .send()
        .await?;

    let resp = resp.json::<Response>().await?;

    for (symbol, currency) in resp.data.into_iter() {
           wtr.write_record(&[currency.name, symbol.to_owned(), currency.quote.0.get("USD").unwrap().price.to_string(), currency.quote.0.get("USD").unwrap().percent_change_24h.to_string(),  currency.quote.0.get("USD").unwrap().percent_change_7d.to_string()])?;
    }

    wtr.flush()?;
    

    Ok(())

}

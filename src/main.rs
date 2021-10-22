
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use clap::{Arg, App};

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

    println!("{:#?}", resp);

    

    Ok(())

}

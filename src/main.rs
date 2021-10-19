
use std::collections::HashMap;

#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use std::env;




#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    
    let mut params = HashMap::new();
    params.insert("symbol", "BTC");
    dotenv().ok();

    
   
    let client = reqwest::Client::new();
    let resp = client
        .get(" https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
        .header("X-CMC_PRO_API_KEY", dotenv!("API_KEY"))
        .query(&params)
        .send()
        .await?;

    let resp = resp.text().await?;

     println!("{:#?}", resp);

    Ok(())

}

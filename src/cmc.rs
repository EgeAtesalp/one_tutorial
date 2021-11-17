use std::collections::HashMap;
use serde::{Serialize, Deserialize};




#[derive(Serialize,Deserialize,Debug)]
pub struct Currency{
    pub name: String,
    pub symbol: String,
    pub quote: Quotes,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Quote{
    pub price: f64,
    pub percent_change_24h: f64,
    pub percent_change_7d: f64,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct CMCResponse{pub data : HashMap<String,Currency>}

#[derive(Serialize,Deserialize,Debug)]
pub struct Quotes(pub HashMap<String, Quote>);
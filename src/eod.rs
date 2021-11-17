use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EODResponse {
    pub ode: String,
    pub close: f64,
}
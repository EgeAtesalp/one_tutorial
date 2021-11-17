



#[derive(Debug)]
pub enum GeneralError{
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
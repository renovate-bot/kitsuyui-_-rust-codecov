/**
 * Error is an enum wrapping all possible errors.
 */
#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    EnvError(std::env::VarError),
    DeserializeError(serde_json::Error),
}

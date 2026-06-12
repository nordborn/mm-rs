pub trait AsksBids {
    fn asks(&self) -> Vec<Vec<String>>;
    fn bids(&self) -> Vec<Vec<String>>;
}

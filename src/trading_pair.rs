use strum_macros::{Display, EnumIter};

#[derive(Clone, Debug, Display, EnumIter)]
pub enum ETradingPair {
    BtcUsdc,
    SolUsdc
}
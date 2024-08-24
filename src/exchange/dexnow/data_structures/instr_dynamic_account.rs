use crate::exchange::dexnow::data_structures::line_px::LinePx;

#[derive(Debug)]
pub struct InstrDynamicAccount {
    pub spot_bids: Vec<LinePx>,
    pub spot_asks: Vec<LinePx>,
}
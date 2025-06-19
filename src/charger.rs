use tokio::sync::mpsc;

use crate::rpc::messages::OcppCall;

pub type ChargerMspcType = OcppCall;
#[derive(Clone)]
pub struct Charger {
    pub _name: String,
    pub tx: mpsc::Sender<ChargerMspcType>,
    pub transaction_id: Option<i32>,
}

impl Charger {
    pub const fn new(name: String, tx: mpsc::Sender<ChargerMspcType>) -> Self {
        Self {
            _name: name,
            tx,
            transaction_id: None,
        }
    }
}

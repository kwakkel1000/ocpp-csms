use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::{
    connector::{Connector, ConnectorInner},
    rpc::messages::OcppCall,
};

pub type ChargerMspcType = OcppCall;

#[derive(Clone, Debug)]
pub struct ChargeSession {
    transaction_id: i32,
}

impl ChargeSession {
    pub const fn new(transaction_id: i32) -> Self {
        Self { transaction_id }
    }

    #[allow(dead_code)]
    pub fn set_transaction_id(&mut self, transaction_id: i32) {
        self.transaction_id = transaction_id;
    }

    pub const fn get_transaction_id(&self) -> i32 {
        self.transaction_id
    }
}

#[derive(Clone, Debug)]
pub struct Charger {
    pub _name: String,
    pub tx: mpsc::Sender<ChargerMspcType>,
    connectors: HashMap<u32, Connector>,
}

impl Charger {
    pub fn new(name: String, tx: mpsc::Sender<ChargerMspcType>) -> Self {
        Self {
            _name: name,
            tx,
            connectors: HashMap::new(),
        }
    }

    pub fn add_connector(&mut self, connector_id: u32) {
        match connector_id {
            0 => self.connectors.insert(
                connector_id,
                Connector::Global(ConnectorInner::new(connector_id)),
            ),
            _ => self.connectors.insert(
                connector_id,
                Connector::Real(ConnectorInner::new(connector_id)),
            ),
        };
    }

    #[allow(dead_code)]
    pub fn get_connector(&self, connector_id: u32) -> Option<&Connector> {
        self.connectors.get(&connector_id)
    }

    pub fn get_connector_mut(&mut self, connector_id: u32) -> Option<&mut Connector> {
        self.connectors.get_mut(&connector_id)
    }

    pub fn get_connectors(&self) -> Vec<u32> {
        self.connectors.keys().copied().collect()
    }
}

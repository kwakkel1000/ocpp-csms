use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::{
    charger::{Charger, ChargerMspcType},
    connector::Connector,
};

#[derive(Clone)]
pub struct Context {
    chargers: HashMap<String, Charger>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            chargers: HashMap::new(),
        }
    }

    pub fn add_charger(&mut self, name: String, tx: mpsc::Sender<ChargerMspcType>) {
        self.chargers.insert(name.clone(), Charger::new(name, tx));
    }

    pub fn del_charger(&mut self, name: &str) {
        self.chargers.remove(name);
    }

    pub fn get_charger(&self, name: &str) -> Option<&Charger> {
        self.chargers.get(name)
    }

    pub fn get_charger_mut(&mut self, name: &str) -> Option<&mut Charger> {
        self.chargers.get_mut(name)
    }

    pub fn start_transaction(&mut self, name: &str, connector_id: u32, transaction_id: i32) {
        if let Some(charger) = self.chargers.get_mut(name) {
            if let Some(Connector::Real(connector)) = charger.get_connector_mut(connector_id) {
                connector.set_session(transaction_id);
            }
        }
    }

    pub fn stop_transaction(&mut self, connector_id: u32, name: &str) -> Option<i32> {
        if let Some(charger) = self.chargers.get_mut(name) {
            if let Some(Connector::Real(connector)) = charger.get_connector_mut(connector_id) {
                if let Some(session) = connector.get_session() {
                    let id = session.get_transaction_id();
                    connector.delete_session();
                    return Some(id);
                }
            }
        }
        None
    }
}

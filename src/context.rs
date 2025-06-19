use std::{collections::HashMap, sync::Arc};

use once_cell::sync::OnceCell;
use tokio::sync::{mpsc, Mutex};

use crate::charger::{Charger, ChargerMspcType};

static CONTEXT: OnceCell<Arc<Context>> = OnceCell::new();

pub struct Context {
    chargers: Arc<Mutex<HashMap<String, Charger>>>,
}

impl Context {
    pub async fn add_charger(&self, name: String, tx: mpsc::Sender<ChargerMspcType>) {
        self.chargers
            .lock()
            .await
            .insert(name.clone(), Charger::new(name, tx));
    }

    pub async fn del_charger(&self, name: &str) {
        self.chargers.lock().await.remove(name);
    }

    pub async fn get_charger(&self, name: &str) -> Option<Charger> {
        let chargers = self.chargers.lock().await;
        if let Some(charger) = chargers.get(name) {
            return Some(charger.clone());
        }
        drop(chargers);
        None
    }

    pub async fn start_transaction(&self, name: &str, transaction_id: i32) {
        let mut chargers = self.chargers.lock().await;
        if let Some(charger) = chargers.get_mut(name) {
            charger.transaction_id = Some(transaction_id);
        }
        drop(chargers);
    }

    pub async fn stop_transaction(&self, name: &str) -> Option<i32> {
        let mut chargers = self.chargers.lock().await;
        if let Some(charger) = chargers.get_mut(name) {
            let id = charger.transaction_id;
            charger.transaction_id = None;
            return id;
        }
        drop(chargers);
        None
    }
}

pub(crate) fn get_context() -> Arc<Context> {
    CONTEXT.get().expect("CONTEXT not set").clone()
}

pub fn init() {
    CONTEXT.get_or_init(|| {
        Arc::new(Context {
            chargers: Arc::new(Mutex::new(HashMap::new())),
        })
    });
}

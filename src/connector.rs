use crate::charger::ChargeSession;

#[derive(Clone, Debug)]
pub enum Connector {
    Global(ConnectorInner),
    Real(ConnectorInner),
}

impl Connector {
    pub fn set_lifetime_energy_usage(&mut self, lifetime_energy_usage: f64) {
        match self {
            Self::Real(connector_inner) | Self::Global(connector_inner) => {
                connector_inner.set_lifetime_energy_usage(lifetime_energy_usage);
            }
        };
    }
}

#[derive(Clone, Debug)]
pub struct ConnectorInner {
    id: u32,
    session: Option<ChargeSession>,
    lifetime_energy_usage: Option<f64>,
    current_offered: Option<f64>,
}

impl ConnectorInner {
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            session: None,
            lifetime_energy_usage: None,
            current_offered: None,
        }
    }

    #[allow(dead_code)]
    pub const fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_session(&mut self, transaction_id: i32) {
        self.session = Some(ChargeSession::new(transaction_id));
    }

    pub const fn get_session(&self) -> Option<&ChargeSession> {
        self.session.as_ref()
    }

    #[allow(dead_code)]
    pub fn get_session_mut(&mut self) -> &mut Option<ChargeSession> {
        &mut self.session
    }

    pub fn delete_session(&mut self) {
        self.session = None;
    }

    pub fn set_lifetime_energy_usage(&mut self, lifetime_energy_usage: f64) {
        self.lifetime_energy_usage = Some(lifetime_energy_usage);
    }

    pub fn set_current_offered(&mut self, current_offered: f64) {
        self.current_offered = Some(current_offered);
    }

    pub const fn get_current_offered(&self) -> Option<f64> {
        self.current_offered
    }
}

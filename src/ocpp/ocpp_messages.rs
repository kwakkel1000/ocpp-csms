use crate::rpc::messages::OcppCall;

impl OcppCall {
    #[allow(unused)]
    pub fn validate(&self) -> Result<(), &str> {
        self.message_type_id_test()?;
        Ok(())
    }

    /// `message_type_id` should be 2 if it's a Call
    #[allow(unused)]
    const fn message_type_id_test(&self) -> Result<(), &'static str> {
        match self.message_type_id {
            2 => Ok(()),
            _ => Err("\"message_type_id\" should be 2 if it's a Call"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::rpc::messages::{OcppCall, OcppMessageType};

    #[test]
    fn validate_call() {
        let json = r#"[2,"19223201","BootNotification",{"chargingStation":{"model":"SingleSocketCharger","vendorName":"VendorX"},"reason":"PowerUp"}]"#;
        let ocpp_message_type = serde_json::from_str::<OcppMessageType>(&json).unwrap();
        let ocpp_call: OcppCall = ocpp_message_type.try_into().unwrap();
        assert_eq!(ocpp_call.validate().is_ok(), true);
    }
}

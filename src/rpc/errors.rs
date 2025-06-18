#[allow(unused)]
pub enum RpcErrorCodes {
    // FormatViolation,
    // GenericError,
    InternalError,
    // MessageTypeNotSupported,
    // NotImplemented,
    // NotSupported,
    // OccurrenceConstraintViolation,
    // PropertyConstraintViolation,
    // ProtocolError,
    RpcFrameworkError,
    // SecurityError,
    // TypeConstraintViolation,
}

impl RpcErrorCodes {
    #[allow(unused)]
    pub const fn description(&self) -> &str {
        match self {
            // Self::FormatViolation => "Payload for Action is syntactically incorrect",
            // Self::GenericError => "Any other error not covered by the more specific error codes in this table",
            Self::InternalError => "An internal error occurred and the receiver was not able to process the requested Action successfully" ,
            // Self::MessageTypeNotSupported => "A message with an Message Type Number received that is not supported by this implementation.",
            // Self::NotImplemented => "Requested Action is not known by receiver",
            // Self::NotSupported => "Requested Action is recognized but not supported by the receiver",
            // Self::OccurrenceConstraintViolation => "Payload for Action is syntactically correct but at least one of the fields violates occurrence constraints",
            // Self::PropertyConstraintViolation => "Payload is syntactically correct but at least one field contains an invalid value",
            // Self::ProtocolError => "Payload for Action is not conform the PDU structure",
            Self::RpcFrameworkError => "Content of the call is not a valid RPC Request, for example: MessageId could not be read.",
            // Self::SecurityError => "During the processing of Action a security issue occurred preventing receiver from completing the Action successfully",
            // Self::TypeConstraintViolation => "Payload for Action is syntactically correct but at least one of the fields violates data type constraints (e.g. \"somestring\": 12)",
        }
    }
}

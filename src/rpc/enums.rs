use rust_ocpp::v1_6::messages::get_configuration::GetConfigurationRequest;
use rust_ocpp::v1_6::messages::get_configuration::GetConfigurationResponse;
use rust_ocpp::v1_6::messages::remote_stop_transaction::RemoteStopTransactionRequest;
use rust_ocpp::v1_6::messages::remote_stop_transaction::RemoteStopTransactionResponse;
use rust_ocpp::v1_6::messages::start_transaction::StartTransactionRequest;
use rust_ocpp::v1_6::messages::start_transaction::StartTransactionResponse;
use rust_ocpp::v1_6::messages::stop_transaction::StopTransactionRequest;
use rust_ocpp::v1_6::messages::stop_transaction::StopTransactionResponse;
use std::fmt;
use std::str::FromStr;
use strum_macros::Display;

use rust_ocpp::v1_6::messages::authorize::AuthorizeRequest;
use rust_ocpp::v1_6::messages::authorize::AuthorizeResponse;
use rust_ocpp::v1_6::messages::boot_notification::BootNotificationRequest;
use rust_ocpp::v1_6::messages::boot_notification::BootNotificationResponse;
use rust_ocpp::v1_6::messages::cancel_reservation::CancelReservationRequest;
use rust_ocpp::v1_6::messages::cancel_reservation::CancelReservationResponse;
use rust_ocpp::v1_6::messages::change_availability::ChangeAvailabilityRequest;
use rust_ocpp::v1_6::messages::change_availability::ChangeAvailabilityResponse;
use rust_ocpp::v1_6::messages::clear_cache::ClearCacheRequest;
use rust_ocpp::v1_6::messages::clear_cache::ClearCacheResponse;
use rust_ocpp::v1_6::messages::clear_charging_profile::ClearChargingProfileRequest;
use rust_ocpp::v1_6::messages::clear_charging_profile::ClearChargingProfileResponse;
use rust_ocpp::v1_6::messages::firmware_status_notification::FirmwareStatusNotificationRequest;
use rust_ocpp::v1_6::messages::firmware_status_notification::FirmwareStatusNotificationResponse;
use rust_ocpp::v1_6::messages::get_composite_schedule::GetCompositeScheduleRequest;
use rust_ocpp::v1_6::messages::get_composite_schedule::GetCompositeScheduleResponse;
use rust_ocpp::v1_6::messages::get_local_list_version::GetLocalListVersionRequest;
use rust_ocpp::v1_6::messages::get_local_list_version::GetLocalListVersionResponse;
use rust_ocpp::v1_6::messages::heart_beat::HeartbeatRequest;
use rust_ocpp::v1_6::messages::heart_beat::HeartbeatResponse;
use rust_ocpp::v1_6::messages::meter_values::MeterValuesRequest;
use rust_ocpp::v1_6::messages::meter_values::MeterValuesResponse;
use rust_ocpp::v1_6::messages::remote_start_transaction::RemoteStartTransactionRequest;
use rust_ocpp::v1_6::messages::remote_start_transaction::RemoteStartTransactionResponse;
use rust_ocpp::v1_6::messages::reserve_now::ReserveNowRequest;
use rust_ocpp::v1_6::messages::reserve_now::ReserveNowResponse;
use rust_ocpp::v1_6::messages::reset::ResetRequest;
use rust_ocpp::v1_6::messages::reset::ResetResponse;
use rust_ocpp::v1_6::messages::send_local_list::SendLocalListRequest;
use rust_ocpp::v1_6::messages::send_local_list::SendLocalListResponse;
use rust_ocpp::v1_6::messages::set_charging_profile::SetChargingProfileRequest;
use rust_ocpp::v1_6::messages::set_charging_profile::SetChargingProfileResponse;
use rust_ocpp::v1_6::messages::status_notification::StatusNotificationRequest;
use rust_ocpp::v1_6::messages::status_notification::StatusNotificationResponse;
use rust_ocpp::v1_6::messages::trigger_message::TriggerMessageRequest;
use rust_ocpp::v1_6::messages::trigger_message::TriggerMessageResponse;
use rust_ocpp::v1_6::messages::unlock_connector::UnlockConnectorRequest;
use rust_ocpp::v1_6::messages::unlock_connector::UnlockConnectorResponse;
use rust_ocpp::v1_6::messages::update_firmware::UpdateFirmwareRequest;
use rust_ocpp::v1_6::messages::update_firmware::UpdateFirmwareResponse;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum AuthorizeKind {
    Request(AuthorizeRequest),
    Response(AuthorizeResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum BootNotificationKind {
    Request(BootNotificationRequest),
    Response(BootNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum CancelReservationKind {
    Request(CancelReservationRequest),
    Response(CancelReservationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ChangeAvailabilityKind {
    Request(ChangeAvailabilityRequest),
    Response(ChangeAvailabilityResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ClearCacheKind {
    Request(ClearCacheRequest),
    Response(ClearCacheResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ClearChargingProfileKind {
    Request(ClearChargingProfileRequest),
    Response(ClearChargingProfileResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum FirmwareStatusNotificationKind {
    Request(FirmwareStatusNotificationRequest),
    Response(FirmwareStatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetCompositeScheduleKind {
    Request(GetCompositeScheduleRequest),
    Response(GetCompositeScheduleResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetConfigurationKind {
    Request(GetConfigurationRequest),
    Response(GetConfigurationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetLocalListVersionKind {
    Request(GetLocalListVersionRequest),
    Response(GetLocalListVersionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum HeartbeatKind {
    Request(HeartbeatRequest),
    Response(HeartbeatResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum MeterValuesKind {
    Request(MeterValuesRequest),
    Response(MeterValuesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ReserveNowKind {
    Request(ReserveNowRequest),
    Response(ReserveNowResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ResetKind {
    Request(ResetRequest),
    Response(ResetResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum RemoteStartTransactionKind {
    Request(RemoteStartTransactionRequest),
    Response(RemoteStartTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum RemoteStopTransactionKind {
    Request(RemoteStopTransactionRequest),
    Response(RemoteStopTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SendLocalListKind {
    Request(SendLocalListRequest),
    Response(SendLocalListResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SetChargingProfileKind {
    Request(SetChargingProfileRequest),
    Response(SetChargingProfileResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum StartTransactionKind {
    Request(StartTransactionRequest),
    Response(StartTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum StopTransactionKind {
    Request(StopTransactionRequest),
    Response(StopTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum StatusNotificationKind {
    Request(StatusNotificationRequest),
    Response(StatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum TriggerMessageKind {
    Request(TriggerMessageRequest),
    Response(TriggerMessageResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum UnlockConnectorKind {
    Request(UnlockConnectorRequest),
    Response(UnlockConnectorResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum UpdateFirmwareKind {
    Request(UpdateFirmwareRequest),
    Response(UpdateFirmwareResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum OcppActionEnum {
    Authorize,
    BootNotification,
    CancelReservation,
    CertificateSigned,
    ChangeAvailability,
    ClearCache,
    ClearChargingProfile,
    FirmwareStatusNotification,
    GetCompositeSchedule,
    GetConfiguration,
    GetLocalListVersion,
    Heartbeat,
    MeterValues,
    RemoteStartTransaction,
    RemoteStopTransaction,
    ReserveNow,
    Reset,
    SendLocalList,
    SetChargingProfile,
    StartTransaction,
    StopTransaction,
    StatusNotification,
    UnlockConnector,
    UpdateFirmware,
}

impl fmt::Display for OcppActionEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for OcppActionEnum {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Authorize" => Ok(Self::Authorize),
            "BootNotification" => Ok(Self::BootNotification),
            "CancelReservation" => Ok(Self::CancelReservation),
            "ChangeAvailability" => Ok(Self::ChangeAvailability),
            "ClearCache" => Ok(Self::ClearCache),
            "ClearChargingProfile" => Ok(Self::ClearChargingProfile),
            "FirmwareStatusNotification" => Ok(Self::FirmwareStatusNotification),
            "GetCompositeSchedule" => Ok(Self::GetCompositeSchedule),
            "GetConfiguration" => Ok(Self::GetConfiguration),
            "GetLocalListVersion" => Ok(Self::GetLocalListVersion),
            "Heartbeat" => Ok(Self::Heartbeat),
            "MeterValues" => Ok(Self::MeterValues),
            "RemoteStartTransaction" => Ok(Self::RemoteStartTransaction),
            "RemoteStopTransaction" => Ok(Self::RemoteStopTransaction),
            "ReserveNow" => Ok(Self::ReserveNow),
            "Reset" => Ok(Self::Reset),
            "SendLocalList" => Ok(Self::SendLocalList),
            "SetChargingProfile" => Ok(Self::SetChargingProfile),
            "StartTransaction" => Ok(Self::StartTransaction),
            "StopTransaction" => Ok(Self::StopTransaction),
            "StatusNotification" => Ok(Self::StatusNotification),
            "UnlockConnector" => Ok(Self::UnlockConnector),
            "UpdateFirmware" => Ok(Self::UpdateFirmware),
            _ => Err(()),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum OcppPayload {
    Authorize(AuthorizeKind),
    BootNotification(BootNotificationKind),
    CancelReservation(CancelReservationKind),
    ChangeAvailability(ChangeAvailabilityKind),
    ClearCache(ClearCacheKind),
    ClearChargingProfile(ClearChargingProfileKind),
    FirmwareStatusNotification(FirmwareStatusNotificationKind),
    GetCompositeSchedule(GetCompositeScheduleKind),
    GetConfiguration(GetConfigurationKind),
    GetLocalListVersion(GetLocalListVersionKind),
    Heartbeat(HeartbeatKind),
    MeterValues(MeterValuesKind),
    ReserveNow(ReserveNowKind),
    Reset(ResetKind),
    RemoteStartTransaction(RemoteStartTransactionKind),
    RemoteStopTransaction(RemoteStopTransactionKind),
    SendLocalList(SendLocalListKind),
    SetChargingProfile(SetChargingProfileKind),
    StartTransaction(StartTransactionKind),
    StopTransaction(StopTransactionKind),
    StatusNotification(StatusNotificationKind),
    TriggerMessage(TriggerMessageKind),
    UnlockConnector(UnlockConnectorKind),
    UpdateFirmware(UpdateFirmwareKind),
}

use crate::enums::MessageType;
use webrtc::ice_transport::ice_candidate::RTCIceCandidateInit;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

#[derive(Clone)]
pub struct Payload {
    r#type: String,
    connection_id: String,
    metadata: String,
    label: String,
    serialization: String,
    reliable: bool,
    candidate: RTCIceCandidateInit,
    sdp: RTCSessionDescription,
    browser: String,
    msg: String,
}

#[derive(Clone)]
pub struct Message {
    r#type: MessageType,
    src: String,
    dst: String,
    payload: Payload,
}

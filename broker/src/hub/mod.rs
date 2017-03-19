
pub mod client_connection;
pub mod client_session;
pub mod local_router;
pub mod router_follower;
pub mod router_leader;
pub mod session_timer;

use std::net::SocketAddr;
use std::time::Instant;

use mqtt::{QualityOfService};
use mqtt::packet::{PublishPacket, SubscribePacket, UnsubscribePacket};


/// Message structure send to `client_connection`
#[derive(Debug, Clone)]
pub enum ClientConnectionMsg {
    Data(SocketAddr, Vec<u8>),
    DisconnectClient(SocketAddr)
    // Shutdown
}

#[derive(Debug, Clone)]
pub enum ClientSessionMsg {
    Data(SocketAddr, Vec<u8>),
    Publish(SocketAddr, QualityOfService, PublishPacket),
    ClientDisconnect(SocketAddr),
    // (user_id, addr, packets, subscribe_qos)
    RetainPackets(u32, SocketAddr, Vec<PublishPacket>, QualityOfService),
    Timeout(SessionTimerPayload)
    // Shutdown
}

#[derive(Debug, Clone)]
pub enum LocalRouterMsg {
    // Forward publish message to `router_follower` or `local_router`
    ForwardPublish(u32, PublishPacket),
    // Receive publish packet from `router_follower` or `local_router`
    Publish(u32, PublishPacket),
    Subscribe(u32, SocketAddr, SubscribePacket),
    Unsubscribe(u32, SocketAddr, UnsubscribePacket),
    ClientDisconnect(u32, SocketAddr),
    // Shutdown
}

#[derive(Debug, Clone)]
pub enum RouterFollowerMsg {
    _Shutdown
}

#[derive(Debug, Clone)]
pub enum RouterLeaderMsg {
    _Shutdown
}

#[derive(Debug, Copy, Clone)]
pub enum SessionTimerAction {
    Set(Instant), Cancel
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum SessionTimerPacketType {
    // [QoS.1.send] Receive PUBACK timeout
    RecvPubackTimeout,
    // [QoS.2.send] Receive PUBREC timeout
    RecvPubrecTimeout,
    // [QoS.2.send] Receive PUBCOMP timeout
    RecvPubcompTimeout,
    // [QoS.2.recv] Receive PUBREL timeout
    RecvPubrelTimeout,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum SessionTimerPayload {
    // SocketAddr => Client addr
    // u16 => packet_identifier (pkid)
    RecvPacketTimer(SessionTimerPacketType, SocketAddr, u16),
    // Receive PINGREQ timeout
    KeepAliveTimer(SocketAddr),
    // Decode one packet timeout (maybe useless ??)
    // DecodePacketTimer(SocketAddr),
}
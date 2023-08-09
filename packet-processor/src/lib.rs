//! # packet-processor
//! This crate provides a simple way to handle packets in a server.

/// `PacketProcessor` is the main trait of the crate. It provides a way to register callbacks for packets.
pub trait PacketProcessor {
    /// Registers all callbacks for packets.
    fn register(&mut self);
    /// Returns a list of supported packets.
    fn supported(&self) -> Vec<proto::PacketId>;
    /// Returns true if the packet is supported.
    fn is_supported(&self, packet_id: &proto::PacketId) -> bool;
    /// Processes a packet.
    fn process(
        &mut self,
        user_id: u32,
        packet_id: proto::PacketId,
        metadata: Vec<u8>,
        data: Vec<u8>,
    );
}

/// register_callback! macro is used to register a callback for a packet.
#[macro_export]
macro_rules! register_callback {
    ($hashmap:ident, $req:ident, $rsp:ident, $handler:ident) => {
        $hashmap.insert(
            proto::PacketId::$req,
            |slef: &mut Self, user_id: u32, metadata: &proto::PacketHead, data: Vec<u8>| {
                let req = proto::$req::decode(&mut std::io::Cursor::new(data)).unwrap();
                let mut rsp = proto::$rsp::default();

                println!("Received REQ {:?}", req);

                slef.$handler(user_id, &metadata, &req, &mut rsp);

                let message =
                    IpcMessage::new_from_proto(proto::PacketId::$rsp, user_id, metadata, &rsp);
                slef.packets_to_send_tx.send(message).unwrap();
            },
        );
    };

    ($hashmap:ident, $notify:ident, $handler:ident) => {
        $hashmap.insert(
            proto::PacketId::$notify,
            |slef: &mut Self, user_id: u32, metadata: &proto::PacketHead, data: Vec<u8>| {
                let notify = proto::$notify::decode(&mut std::io::Cursor::new(data)).unwrap();
                println!("Received NOTIFY {:?}", notify);

                slef.$handler(user_id, &metadata, &notify);
            },
        );
    };
}

/// build_and_send! macro is used to build a packet and send it out
#[macro_export]
macro_rules! build_and_send {
    ($self:ident, $user_id: ident, $metadata:ident, $id:ident { $($i:ident : $e:expr,)* }) => {{
        $self.packets_to_send_tx.send(
            IpcMessage::new_from_proto(
                proto::PacketId::$id,
                $user_id,
                $metadata,
                &proto::$id { $($i: $e,)* ..proto::$id::default() }
            )
        ).unwrap();
    }};
}

/// build! macro is used to build a packet
#[macro_export]
macro_rules! build {
    ($id:ident { $($i:ident : $e:expr,)* }) => {{
        proto::$id { $($i: $e,)* ..proto::$id::default() }
    }};
}

/*
#[macro_export]
macro_rules! unpack {
    ($packet:ident, $buffer:ident) => {{
        proto::$packet::decode(&mut std::io::Cursor::new($buffer)).unwrap()
    }};
}
*/

pub trait EasilyUnpackable {
    fn from(buf: &[u8]) -> Self;
}

impl<T: prost::Message + std::default::Default> EasilyUnpackable for T {
    fn from(buf: &[u8]) -> Self {
        Self::decode(&mut std::io::Cursor::new(buf)).unwrap()
    }
}

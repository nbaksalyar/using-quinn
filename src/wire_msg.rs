use bincode;
use bytes::Bytes;
use std::net::SocketAddr;

/// Final type serialised and sent on the wire by Crust
#[derive(Serialize, Deserialize, Debug)]
pub enum WireMsg {
    CertificateDer(Vec<u8>),
    EndpointEchoReq,
    EndpointEchoResp(SocketAddr),
    UserMsg(Bytes),
}

impl Into<Bytes> for WireMsg {
    fn into(self) -> Bytes {
        From::from(unwrap!(bincode::serialize(&self)))
    }
}

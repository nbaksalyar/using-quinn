use std::io;
use std::net::SocketAddr;

quick_error! {
    #[derive(Debug)]
     pub enum Error {
         IoError(e: io::Error) {
             display("IO Error: {}", e)
             from()
         }
         ReadError(e: quinn::ReadError) {
             display("Read Error: {}", e)
             from()
         }
         BiDirectionalStreamAttempted(peer_addr: SocketAddr) {
             display("Bi-directional stream attempted by peer {}", peer_addr)
         }
         ConnectError(e: quinn::ConnectError) {
             display("Connection Error: {}", e)
             from()
         }
         ConnectionError(e: quinn::ConnectionError) {
             display("Connection Error: {}", e)
             from()
         }
         EndpointError(e: quinn::EndpointError) {
             display("Endpoint error: {}", e)
             from()
         }
         CertificateParseError(e: quinn::tls::ParseError) {
             display("Certificate Parse Error: {}", e)
             from()
         }
         DuplicateConnectionToPeer(peer_addr: SocketAddr) {
             display("Duplicate connection attempted to peer {}", peer_addr)
         }
         NoEndpointEchoServerFound {
             display("There's no endpoint echo server with a Global Address to ask.")
         }
         ListenerNotInitialised {
            display("Listener is not yet initialised.")
         }
     }
}

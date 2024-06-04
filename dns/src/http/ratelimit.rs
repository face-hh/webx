use actix_governor::{KeyExtractor, SimpleKeyExtractionError};
use actix_web::{dev::ServiceRequest, web};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RealIpKeyExtractor;

impl KeyExtractor for RealIpKeyExtractor {
    type Key = IpAddr;
    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        let reverse_proxy_ip = req
            .app_data::<web::Data<IpAddr>>()
            .map(|ip| ip.get_ref().to_owned())
            .unwrap_or_else(|| IpAddr::from_str("0.0.0.0").unwrap());

        let peer_ip = req.peer_addr().map(|socket| socket.ip());
        let connection_info = req.connection_info();

        match peer_ip {
            Some(peer) if peer == reverse_proxy_ip => connection_info
                .realip_remote_addr()
                .ok_or_else(|| SimpleKeyExtractionError::new("Could not extract real IP address from request"))
                .and_then(|str| {
                    SocketAddr::from_str(str)
                        .map(|socket| socket.ip())
                        .or_else(|_| IpAddr::from_str(str))
                        .map_err(|_| SimpleKeyExtractionError::new("Could not extract real IP address from request"))
                }),
            _ => connection_info
                .peer_addr()
                .ok_or_else(|| SimpleKeyExtractionError::new("Could not extract peer IP address from request"))
                .and_then(|str| SocketAddr::from_str(str).map_err(|_| SimpleKeyExtractionError::new("Could not extract peer IP address from request")))
                .map(|socket| socket.ip()),
        }
    }
}

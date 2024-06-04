use super::models::Ratelimit;
use actix_web::{dev::ServiceRequest, web, HttpResponse, HttpResponseBuilder};

use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_governor::{
    governor::clock::{Clock, DefaultClock, QuantaInstant},
    governor::NotUntil,
    KeyExtractor, SimpleKeyExtractionError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RealIpKeyExtractor;

impl KeyExtractor for RealIpKeyExtractor {
    type Key = IpAddr;
    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        let reverse_proxy_ip = req
            .app_data::<web::Data<super::AppState>>()
            .map(|ip| ip.get_ref().trusted.to_owned())
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

    fn exceed_rate_limit_response(&self, negative: &NotUntil<QuantaInstant>, mut response: HttpResponseBuilder) -> HttpResponse {
        let current_unix_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
        let wait_time = negative.wait_time_from(DefaultClock::default().now()).as_secs();
        let wait_time_unix = current_unix_timestamp + negative.wait_time_from(DefaultClock::default().now()).as_secs();

        response.json(Ratelimit {
            after: wait_time_unix,
            error: "ratelimited_endpoint",
            msg: format!("Too many requests, try again in {wait_time}s"),
        })
    }
}

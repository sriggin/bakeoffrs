use std::sync::Arc;
use tonic::{Request, Response, Status};
use crate::{RequestValidator, TrafficScanner, TrafficScanRequest, TrafficScanResponse};

#[derive(Clone)]
pub struct TrafficScannerImpl {
    request_validator: Arc<RequestValidator>,
}

impl TrafficScannerImpl {
    pub fn new(request_validator: RequestValidator) -> TrafficScannerImpl {
        let arc = Arc::new(request_validator);
        TrafficScannerImpl { request_validator: arc }
    }
}

#[tonic::async_trait]
impl TrafficScanner for TrafficScannerImpl {
    async fn is_traffic_valid(
        &self,
        request: Request<TrafficScanRequest>,
    ) -> Result<Response<TrafficScanResponse>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let req = request.get_ref();
        let user_agent = req.user_agent.as_str();
        let ip = req.ip.as_str();

        self.request_validator.is_valid(user_agent, ip).await
            .map_err(|e| {
                Status::internal(e.to_string())
            })
            .map(|is_valid| {
                Response::new(TrafficScanResponse { is_valid })
            })
    }
}

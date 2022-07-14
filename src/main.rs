mod request_validation;
mod traffic_scanner;

use std::error::Error;
use std::path::Path;
use tonic::{transport::Server};

use crate::ivt::traffic_scanner_server::{TrafficScanner, TrafficScannerServer};
use crate::ivt::{TrafficScanRequest, TrafficScanResponse};

use crate::request_validation::*;
use crate::traffic_scanner::*;

mod ivt {
    include!("ivt.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("greeter_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    println!("Traffic Scanner server listening on {}", addr);

    // Add this
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(ivt::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let request_validator = RequestValidator::load(Path::new("model.cbm"))?;
    request_validator.is_valid("", "").await.unwrap(); // force evaluation, in case loading a model doesn't work

    Server::builder()
        .add_service(TrafficScannerServer::new(TrafficScannerImpl::new(request_validator)))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
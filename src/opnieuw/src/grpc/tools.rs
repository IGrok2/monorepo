use crate::grpc::all::QueryResponse;
use crate::templates::error::internal_error;
use crate::GA;
use tonic::{Response, Status};

pub fn success() -> Result<Response<QueryResponse>, Status> {
    GA.grpc.success.inc();

    Ok(Response::new(QueryResponse {
        success: true,
        message: "Successful action!".to_string(),
    }))
}

pub fn fail(message: &str) -> Result<Response<QueryResponse>, Status> {
    GA.grpc.fail.inc();

    internal_error(message);

    Ok(Response::new(QueryResponse {
        success: false,
        message: message.to_string(),
    }))
}

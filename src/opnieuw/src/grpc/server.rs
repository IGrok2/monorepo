use crate::grpc::{
    all::big_baller_server::BigBallerServer,
    handler::FortyTwo,
};
use std::error::Error;
use tonic::transport::{
    server::ServerTlsConfig,
    Identity,
    Server,
};

pub async fn start_grpc() {
    // because we can't share errors across tasks
    match begin_server().await {
        Ok(_) => {}
        Err(e) => {
            panic!("grpc failed to start: {:?}", e.source())
        }
    }
}

async fn begin_server() -> Result<(), Box<dyn Error>> {
    let meaning = FortyTwo::default();

    Server::builder()
        .add_service(BigBallerServer::new(meaning.clone()))
        .serve("0.0.0.0:4206".parse()?)
        .await?;

    Ok(())
}

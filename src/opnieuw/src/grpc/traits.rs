use crate::grpc::all::{
    AllDomainSchema,
    DeleteDomainSchema,
    FullDomainSchema,
    PartialDomainSchema,
    QueryResponse,
};
use tonic::{
    Request,
    Response,
    Status,
};

/*
 // on startup message, includes all domains
 rpc AllDomains(AllDomainSchema) returns (QueryResponse);
 // update domain after the user, or an admin, makes some changes
 rpc UpdateDomain(PartialDomainSchema) returns (QueryResponse);
 // new domain on our service
 rpc NewDomain(AllDomainSchema) returns (QueryResponse);
 // deleting a domain from our service
 rpc DeleteDomain(DeleteDomainSchema) returns (QueryResponse);
*/

#[tonic::async_trait]
pub trait AllDomains {
    async fn all_domains(
        &self,
        request: Request<AllDomainSchema>,
    ) -> Result<Response<QueryResponse>, Status>;
}

#[tonic::async_trait]
pub trait UpdateDomain {
    async fn update_domain(
        &self,
        request: Request<PartialDomainSchema>,
    ) -> Result<Response<QueryResponse>, Status>;
}

#[tonic::async_trait]
pub trait NewDomain {
    async fn new_domain(
        &self,
        request: Request<FullDomainSchema>,
    ) -> Result<Response<QueryResponse>, Status>;
}

#[tonic::async_trait]
pub trait DeleteDomain {
    async fn delete_domain(
        &self,
        request: Request<DeleteDomainSchema>,
    ) -> Result<Response<QueryResponse>, Status>;
}

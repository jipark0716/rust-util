use tonic::{Request, Response, Status};
use health_check::service_server::Service;
use health_check::{Request as HelloRequest, Response as HelloResponse, SuccessResponse};
use health_check::response::Payload;
use crate::server::health_check::health_check::service_server::ServiceServer;

mod health_check {
    tonic::include_proto!("health_check");
}

pub struct ServerService;

impl ServerService {
    pub fn new() -> ServiceServer<ServerService> {
        ServiceServer::new(Self {
            
        })
    }
}

#[tonic::async_trait]
impl Service for ServerService {
    async fn run(
        &self,
        _request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let id = uuid::Uuid::now_v7();
        let (id1, id2) = id.as_u64_pair();

        Ok(Response::new(HelloResponse {
            id1,
            id2,
            payload: Some(
                Payload::Success(SuccessResponse {})),
        }))
    }
}
pub mod include;

use crate::chat::Handler;
use include::couscous::couscous_server::Couscous;
use include::couscous::{Chat, Deletion, MessageRequest, MessageResponse, NewChannelRequest};
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct Rpc {
    pub chat: Arc<Handler<String>>,
}

#[tonic::async_trait]
impl Couscous for Rpc {
    async fn new_channel(&self, _: Request<NewChannelRequest>) -> Result<Response<Chat>, Status> {
        let handler = self.chat.clone();

        match handler.new_chat().await {
            Ok(uid) => Ok(Response::new(Chat { id: uid })),
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }

    async fn discuss(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let handler = self.chat.clone();
        let params = request.into_inner();

        match handler.send_message(&params.chat_id, &params.message).await {
            Ok(message) => Ok(Response::new(MessageResponse { message })),
            Err(err) => Err(Status::internal(err.to_string()))
        }
    }

    async fn delete_channel(&self, _: Request<Chat>) -> Result<Response<Deletion>, Status> {
        unimplemented!()
    }
}

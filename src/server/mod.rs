pub mod include;
mod msg;

use crate::chat::Handler;
use include::couscous::couscous_server::Couscous;
use include::couscous::{Chat, MessageRequest, MessageResponse, NewChannelRequest};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
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

    type DiscussStream = ReceiverStream<Result<MessageResponse, Status>>;

    async fn discuss(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<Self::DiscussStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let handler = self.chat.clone();
        let params = request.into_inner();

        tokio::spawn(async move {
            handler
                .send_message(&params.chat_id, &params.message, tx)
                .await
                .unwrap();
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

use super::include::couscous::MessageResponse;

impl From<String> for MessageResponse {
    fn from(value: String) -> Self {
        MessageResponse { message: value }
    }
}

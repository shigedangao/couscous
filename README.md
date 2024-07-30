# Couscous 🍽️

Just a small toy gRPC service to interact with LLama or any other AI Model exposed through the [kalosm library](https://github.com/floneum/floneum/tree/main/interfaces/kalosm). For the time being only llama_7b_chat is set to play with this.

The cargo.toml specified metal for osx performance improvement.

# What can we do ?

## Create a new chat

The API allows to create multiple chats. You can create a chat by querying the grpc endpoint

```curl
grpcurl -plaintext 127.0.0.1:50051 couscous.Couscous/NewChannel
```

The response would be

```json
{
  "id": "4f17ed18-b34d-43bf-b865-1db64ab926b9"
}
```

## Sending message to a chat

```curl
grpcurl -plaintext -d '{"chat_id": "25af8332-a15c-4962-abdb-924dce5c4a0d", "message": "Hello how are you today ?"}' 127.0.0.1:50051 couscous.Couscous/Discuss
```

The response would be

```json
{
  "message": "Hello! I'm just an AI, so I don't have feelings like humans do, but thank you for asking! *smiles* It's nice to chat with you. How about you? Is there something on your mind that you'd like to talk about or ask me?"
}
```

## Chat restoration

As the library allows to load the history. All chat will be saved and restored when relaunching the couscous binary.

## Issue

Checking why the model isn't loaded into the GPU memory
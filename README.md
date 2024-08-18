# Couscous 🍽️

Just a small toy gRPC service to interact with LLama or any other AI Model exposed through the [kalosm library](https://github.com/floneum/floneum/tree/main/interfaces/kalosm) or through [ollama](https://github.com/ollama/ollama?tab=readme-ov-file) by using the binding provided by [ollama-rs](https://github.com/pepperoni21/ollama-rs)

The cargo.toml specified metal for osx performance improvement.

## Run

By default couscous will uses Kalosm to run the Llama model. If you wish to use ollama please run the project by running the following command

```sh
cargo run --features ollama
```

## Environment variables

You can set environment varialbles through an .env file. Below is a configuration example (external model is only support for Ollama)

```sh
OLLAMA_HOST="https://{pod-id}-11434.proxy.runpod.net/"
OLLAMA_PORT="443"
GRPC_SERVER_ADDRESS="127.0.0.1:50051"
```

### Note on Ollama

In order to use ollama, please download ollama and download the model llama3.1

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

> [!NOTE]
> If you decide to switch from Kalosm to Ollama or the other way. You'll need to remove the cache. You can do that by deleting the file chats.json

## UI

A minimal ui is available if you want to interact through your browser instead of using postman or grpcurl. You can run the UI by going to the `ui` folder and run the command

```sh
npm run dev
```

> [!WARNING]
> You must run the server first as it'll create a new channel automatically

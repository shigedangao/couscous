import { defineStore } from 'pinia'
import { createPromiseClient, ConnectError } from '@connectrpc/connect'
import { createGrpcWebTransport } from '@connectrpc/connect-web'
import { Couscous } from '../pb/couscous_connect'
import { uid } from 'uid'

const transport = createGrpcWebTransport({
  baseUrl: 'http://127.0.0.1:50051'
})

// Create the rpc client handler
const client = createPromiseClient(Couscous, transport)

export enum Sender {
  Onwer,
  Remote
}

export interface Message {
  message: string
  sender: Sender
  id: string
  done?: boolean
}

export const useChatStore = defineStore('chat', {
  state: () => ({
    messages: [] as Message[],
    currentMessage: {
      message: '',
      sender: Sender.Remote,
      id: uid(),
      done: false
    } as Message,
    channelId: ''
  }),
  getters: {
    getAllMessages: (state) => state.messages,
    getNextMessage: (state) => state.messages[state.messages.length - 1]
  },
  actions: {
    async sendMessage(msg: string) {
      this.messages.push({
        message: msg,
        sender: Sender.Onwer,
        id: uid()
      })

      const asyncIterator = client.discuss({
        message: msg,
        chatId: this.channelId
      })

      for await (const m of asyncIterator) {
        this.currentMessage.message = this.currentMessage.message.concat(m.message)
      }

      this.currentMessage.done = true
      this.messages.push(JSON.parse(JSON.stringify(this.currentMessage)))

      // Reset current message value
      this.currentMessage.message = ''
      this.currentMessage.done = false
    },
    async createNewChannel() {
      try {
        const channel = await client.newChannel({})
        this.channelId = channel.id
      } catch (err) {
        const connectErr = ConnectError.from(err)
        console.log(connectErr)
      }
    }
  }
})

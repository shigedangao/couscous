<template>
  <div id="wrapper">
    <h1>Couscous</h1>
    <div id="chat-result">
      <vue-markdown-it
        v-for="item in messages"
        :key="item.id"
        :class="getTextClass(item)"
        :source="item.message"
      />
      <p v-if="!currentMessage.done" class="remote">{{ currentMessage.message }}</p>
    </div>
    <label for="search">Input anything you wanna say to a chat AI</label>
    <input
      type="text"
      id="search"
      name="search"
      @keyup.enter="dispatchMessage"
      v-model="queryInput"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useChatStore, Sender, type Message } from '@/stores/chat'
import { storeToRefs } from 'pinia'
import { VueMarkdownIt } from '@f3ve/vue-markdown-it'

// Ref
const queryInput = ref('')

// Initialize the store
const store = useChatStore()

// Create a ref from the stores
const { messages, currentMessage } = storeToRefs(store)

// Create a new channel
await store.createNewChannel()

const dispatchMessage = () => {
  store.sendMessage(queryInput.value)

  queryInput.value = ''
}

const getTextClass = (item: Message): string => {
  if (item.sender == Sender.Onwer) {
    return 'owner'
  }

  return 'remote'
}
</script>

<style>
#wrapper {
  width: 100%;
}

label {
  display: block;
}

input {
  width: 100%;
  height: 35px;
  border: 1px solid black;
  border-radius: 30px;
  padding: 20px;
}

#chat-result {
  height: 70vh;
  max-height: 70vh;
  display: flex;
  flex-direction: column;
  overflow-y: scroll;
}

.owner {
  max-width: 40vw;
  padding: 20px;
  align-self: flex-start;
}

.remote {
  max-width: 45vw;
  padding: 20px;
  align-self: flex-end;
}
</style>

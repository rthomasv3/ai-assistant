<template>
    <div class="messages">
        <div v-for="message in messages" :class="message.isUser ? 'user-message': 'bot-message'">
            <p>{{ message.text }}</p>
        </div>
        <div ref="messageAnchor"></div>
    </div>

    <form class="row" @submit.prevent="sendMessage">
        <input id="send-input" v-model="message" placeholder="Send message..." autocomplete="off" :disabled="formDisabled" />
        <button type="submit" :disabled="formDisabled">Send</button>
    </form>
</template>

<script setup>
import { ref, computed, watch } from "vue";
import { useStore } from "vuex";
import { invoke } from "@tauri-apps/api/tauri";

const messageAnchor = ref(null)
const modelResponse = ref("");
const message = ref("");
const formDisabled = ref(false);
const store = useStore();

const messages = computed(() => {
    return store.state.messages;
});

const messageCount = computed(() => {
    return store.state.messages.length;
});

watch(messageCount, () => {
    scrollToBottom();
},{ flush: "post" });

async function sendMessage() {
    store.commit("addMessage", {
        isUser: true,
        text: message.value
    });

    store.commit("addMessage", {
        isUser: false,
        text: "..."
    });

    let userInput = message.value;

    formDisabled.value = true;
    message.value = "";

    modelResponse.value = await invoke("send_message", { text: userInput });

    store.commit("removeLast");

    store.commit("addMessage", {
        isUser: false,
        text: modelResponse.value
    });

    formDisabled.value = false;

    scrollToBottom();
}

function scrollToBottom() {
    setTimeout(function() {
        messageAnchor.value.scrollIntoView({behavior: "smooth"});
    }, (250));
}
</script>
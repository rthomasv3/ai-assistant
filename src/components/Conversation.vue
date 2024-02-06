<template>
    <Header :formDisabled="formDisabled" :initialized="initialized" :loadedModel="loadedModel" @loading="formDisabled = true" @loadingComplete="formDisabled = false" />

    <div class="messages">
        <div v-for="message in messages" :class="message.isUser ? 'user-message' : 'bot-message'">
            <div v-html="message.text"></div>
        </div>
        <div ref="messageAnchor"></div>
    </div>

    <form class="row" @submit.prevent="sendMessage">
        <input id="send-input" v-model="message" placeholder="Send message..." autocomplete="off" :disabled="formDisabled" @keydown.enter.prevent="sendMessage" />
        <button type="submit" :disabled="formDisabled">Send</button>
    </form>
</template>

<script setup>
import Header from "./Header.vue";
import { ref, computed, watch, onBeforeMount } from "vue";
import { useStore } from "vuex";
import { invoke } from "@tauri-apps/api/tauri";
import markdownit from "markdown-it";
import hljs from 'highlight.js'
import 'highlight.js/styles/github-dark.min.css';

const messageAnchor = ref(null)
const modelResponse = ref("");
const message = ref("");
const formDisabled = ref(true);
const initialized = ref(false);
const loadedModel = ref(false);
const store = useStore();
const md = markdownit({
  highlight: function (str, lang) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(str, { language: lang }).value;
      } catch (__) {}
    }

    return ''; // use external default escaping
  }
});

const messages = computed(() => {
    return store.state.messages;
});

const messageCount = computed(() => {
    return store.state.messages.length;
});

onBeforeMount(async () => {
    loadedModel.value = await invoke("initialize");
    formDisabled.value = false;
    initialized.value = true;
});

watch(messageCount, () => {
    scrollToBottom();
}, { flush: "post" });

async function sendMessage() {
    if (message.value.trim() !== "") {
        store.commit("addMessage", {
            isUser: true,
            text: renderAsMarkdown(message.value)
        });

        store.commit("addMessage", {
            isUser: false,
            text: "..."
        });

        let userInput = message.value;

        formDisabled.value = true;
        message.value = "";

        var error = null;
        modelResponse.value = await invoke("send_message", { text: userInput })
        .catch(e => {
            console.error(e);
            error = e;
        });

        if (error === null) {
            store.commit("removeLastMessage");

            store.commit("addMessage", {
                isUser: false,
                text: renderAsMarkdown(modelResponse.value)
            });

            scrollToBottom();
        } else {
            store.commit("removeLastMessage");
            store.commit("removeLastMessage");
        }

        formDisabled.value = false;
    }
}

function scrollToBottom() {
    setTimeout(function () {
        messageAnchor.value.scrollIntoView({ behavior: "smooth" });
    }, (250));
}

function renderAsMarkdown(text) {
    return md.render(text);
}
</script>

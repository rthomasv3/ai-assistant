import { createApp } from "vue";
import { createStore } from 'vuex'
import "./styles.css";
import App from "./App.vue";

const store = createStore({
    state() {
        return {
            messages: []
        }
    },
    mutations: {
        addMessage(state, message) {
            state.messages.push(message);
        },
        removeLastMessage(state) {
            state.messages.pop();
        },
        resetConversation(state) {
            state.messages = [];
        }
    }
})

createApp(App).use(store).mount("#app");

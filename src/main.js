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
        removeLast(state) {
            state.messages.pop();
        }
    }
})

createApp(App).use(store).mount("#app");

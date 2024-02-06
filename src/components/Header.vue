<template>
    <div class="header">
        <div class="header-start"></div>

        <div class="header-center">
            <select :disabled="formDisabled" v-model="selected_path" @change="updateModel">
                <option v-for="model_path, index in model_paths" :value=index>
                    {{ getFileName(model_path) }}
                </option>
            </select>

            <button class="icon-button ml-1" @click.prevent="getModels" :disabled="formDisabled">
                <svg width="18" height="18" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                    <path d="M105.1 202.6c7.7-21.8 20.2-42.3 37.8-59.8c62.5-62.5 163.8-62.5 226.3 0L386.3 160H352c-17.7 0-32 14.3-32 32s14.3 32 32 32H463.5c0 0 0 0 0 0h.4c17.7 0 32-14.3 32-32V80c0-17.7-14.3-32-32-32s-32 14.3-32 32v35.2L414.4 97.6c-87.5-87.5-229.3-87.5-316.8 0C73.2 122 55.6 150.7 44.8 181.4c-5.9 16.7 2.9 34.9 19.5 40.8s34.9-2.9 40.8-19.5zM39 289.3c-5 1.5-9.8 4.2-13.7 8.2c-4 4-6.7 8.8-8.1 14c-.3 1.2-.6 2.5-.8 3.8c-.3 1.7-.4 3.4-.4 5.1V432c0 17.7 14.3 32 32 32s32-14.3 32-32V396.9l17.6 17.5 0 0c87.5 87.4 229.3 87.4 316.7 0c24.4-24.4 42.1-53.1 52.9-83.7c5.9-16.7-2.9-34.9-19.5-40.8s-34.9 2.9-40.8 19.5c-7.7 21.8-20.2 42.3-37.8 59.8c-62.5 62.5-163.8 62.5-226.3 0l-.1-.1L125.6 352H160c17.7 0 32-14.3 32-32s-14.3-32-32-32H48.4c-1.6 0-3.2 .1-4.8 .3s-3.1 .5-4.6 1z"/>
                </svg>
            </button>
        </div>

        <div class="header-end">
            <button class="icon-button mr-1" @click.prevent="reset" :disabled="formDisabled">
                <svg width="18" height="18" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                    <path d="M566.6 54.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0l-192 192-34.7-34.7c-4.2-4.2-10-6.6-16-6.6c-12.5 0-22.6 10.1-22.6 22.6v29.1L364.3 320h29.1c12.5 0 22.6-10.1 22.6-22.6c0-6-2.4-11.8-6.6-16l-34.7-34.7 192-192zM341.1 353.4L222.6 234.9c-42.7-3.7-85.2 11.7-115.8 42.3l-8 8C76.5 307.5 64 337.7 64 369.2c0 6.8 7.1 11.2 13.2 8.2l51.1-25.5c5-2.5 9.5 4.1 5.4 7.9L7.3 473.4C2.7 477.6 0 483.6 0 489.9C0 502.1 9.9 512 22.1 512l173.3 0c38.8 0 75.9-15.4 103.4-42.8c30.6-30.6 45.9-73.1 42.3-115.8z"/>
                </svg>
            </button>

            <button class="icon-button" @click.prevent="showSettingsDialog" :disabled="formDisabled">
                <svg width="18" height="18" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                    <path d="M495.9 166.6c3.2 8.7 .5 18.4-6.4 24.6l-43.3 39.4c1.1 8.3 1.7 16.8 1.7 25.4s-.6 17.1-1.7 25.4l43.3 39.4c6.9 6.2 9.6 15.9 6.4 24.6c-4.4 11.9-9.7 23.3-15.8 34.3l-4.7 8.1c-6.6 11-14 21.4-22.1 31.2c-5.9 7.2-15.7 9.6-24.5 6.8l-55.7-17.7c-13.4 10.3-28.2 18.9-44 25.4l-12.5 57.1c-2 9.1-9 16.3-18.2 17.8c-13.8 2.3-28 3.5-42.5 3.5s-28.7-1.2-42.5-3.5c-9.2-1.5-16.2-8.7-18.2-17.8l-12.5-57.1c-15.8-6.5-30.6-15.1-44-25.4L83.1 425.9c-8.8 2.8-18.6 .3-24.5-6.8c-8.1-9.8-15.5-20.2-22.1-31.2l-4.7-8.1c-6.1-11-11.4-22.4-15.8-34.3c-3.2-8.7-.5-18.4 6.4-24.6l43.3-39.4C64.6 273.1 64 264.6 64 256s.6-17.1 1.7-25.4L22.4 191.2c-6.9-6.2-9.6-15.9-6.4-24.6c4.4-11.9 9.7-23.3 15.8-34.3l4.7-8.1c6.6-11 14-21.4 22.1-31.2c5.9-7.2 15.7-9.6 24.5-6.8l55.7 17.7c13.4-10.3 28.2-18.9 44-25.4l12.5-57.1c2-9.1 9-16.3 18.2-17.8C227.3 1.2 241.5 0 256 0s28.7 1.2 42.5 3.5c9.2 1.5 16.2 8.7 18.2 17.8l12.5 57.1c15.8 6.5 30.6 15.1 44 25.4l55.7-17.7c8.8-2.8 18.6-.3 24.5 6.8c8.1 9.8 15.5 20.2 22.1 31.2l4.7 8.1c6.1 11 11.4 22.4 15.8 34.3zM256 336a80 80 0 1 0 0-160 80 80 0 1 0 0 160z"/>
                </svg>
            </button>
        </div>
    </div>

    <Settings v-if="showSettings" @close="hideSettingsDialog" @saved="settingsSaved" />
</template>

<script setup>
import Settings from "./Settings.vue";
import { ref, watch } from "vue";
import { useStore } from "vuex";
import { invoke } from "@tauri-apps/api/tauri";

const emit = defineEmits(["loading", "loadingComplete"])

const props = defineProps(["initialized", "loadedModel", "formDisabled"])
const initialized = ref(false);
const showSettings = ref(false);
const model_paths = ref([ "No Models" ]);
const selected_path = ref(0);
const store = useStore();

watch(() => props.initialized, async () => {
    if (initialized.value === false && props.initialized === true) {
        await getModels();

        if (props.loadedModel) {
            var config = await invoke("get_config");

            if (config.model_path) {
                var index = model_paths.value.indexOf(config.model_path);
                if (index > -1) {
                    selected_path.value = index;
                }
            }
        } else {
            this.model_paths.splice(0, 0, "No Model Loaded");
            selected_path.value = 0;
        }

        initialized.value = true;
    }
}, { deep: true });

async function getModels() {
    var models = await invoke("get_models");

    if (models.length > 0) {
        model_paths.value = models;
    } else {
        model_paths.value = [ "No Models" ];
    }
}

function getFileName(filePath) {
    return filePath.replace(/^.*[\\/]/, '');
}

async function updateModel() {
    if (model_paths.value.length > selected_path.value) {
        var path = model_paths.value[selected_path.value];
        emit("loading");
        await invoke("update_model", { modelPath: path });
        emit("loadingComplete");
    }
}

async function reset() {
    await invoke("reset_conversation");
    store.commit("resetConversation");
}

function showSettingsDialog() {
    showSettings.value = true;
}

function hideSettingsDialog() {
    showSettings.value = false;
}

async function settingsSaved() {
    showSettings.value = false;
    
}
</script>

<style scoped>
.header {
    padding: 1em 0em 1em 0em;
    display: flex;
    border-bottom: 1px solid #525252;
}

select {
    padding: 0.75em 1em 0.75em 0.75em;
    border-radius: 8px;
    align-self: center;
    width: 100%;
}

option {
    padding: 2.5em;
    border-radius: 0px;
    min-height: 5em;
}

.header-start {
    display: flex; 
    flex-basis: 33.33%; 
    justify-content: end;
}

.header-center {
    display: flex; 
    flex-basis: 33.33%; 
    justify-content: center;
}

.header-end {
    display: flex; 
    flex-basis: 33.33%; 
    justify-content: end;
}

.ml-1{
    margin-left: 10px;
}

.mr-1{
    margin-right: 10px;
}

@media (prefers-color-scheme: dark) {
  select {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}
</style>
<template>
    <fieldset :disabled="loading">
        <div class="dialog-mask" @click.self="close">
            <div class="dialog">
                <div class="dialog-header">
                    <span class="dialog-title">Settings</span>

                    <div class="dialog-close">
                        <button class="icon-button close-button" @click="close">
                            <svg width="16" height="16" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512">
                                <path d="M342.6 150.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L192 210.7 86.6 105.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L146.7 256 41.4 361.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L192 301.3 297.4 406.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L237.3 256 342.6 150.6z"/>
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="dialog-body">
                    <div class="dialog-row">
                        <p>Model Folder</p>
                        <input type="text" v-model="modelFolder" readonly/>
                        <button class="browse-button" @click="selectModelFolder">Browse</button>
                    </div>

                    <div class="dialog-row">
                        <p>Prefer Mmap</p>
                        <input type="checkbox" v-model="preferMmap" />
                    </div>

                    <div class="dialog-row">
                        <p>Context Size</p>
                        <input type="number" v-model="contextSize" />
                    </div>

                    <div class="dialog-row">
                        <p>Use GPU</p>
                        <input type="checkbox" v-model="useGpu" />
                    </div>

                    <div class="dialog-row">
                        <p>GPU Layers</p>
                        <input type="number" v-model="gpuLayers" />
                    </div>
                </div>

                <div class="dialog-footer">
                    <button @click="save">{{ loading ? 'Saving...' : 'Save'}}</button>
                    <button class="cancel-button" @click="close">Cancel</button>
                </div>
            </div>
        </div>
    </fieldset>
</template>

<script setup>
import { ref, onBeforeMount, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'

const emit = defineEmits(["close", "saved"])

const modelFolder = ref("");
const modelPath = ref("");
const preferMmap = ref(false);
const contextSize = ref(0);
const useGpu = ref(false);
const gpuLayers = ref(null);
const loading = ref(true);

var unlisten = undefined;

onBeforeMount(async () => {
    let config = await invoke("get_config");
    modelFolder.value = config.model_folder;
    modelPath.value = config.model_path;
    preferMmap.value = config.prefer_mmap;
    contextSize.value = config.context_size;
    useGpu.value = config.use_gpu;
    gpuLayers.value = config.gpu_layers;

    unlisten = await listen('folder_picked', (event) => {
        modelFolder.value = event.payload;
    });

    loading.value = false;
});

onBeforeUnmount(() => {
    if (unlisten) {
        unlisten();
    }
});

function selectModelFolder() {
    invoke("pick_folder");
}

async function save() {
    loading.value = true;
    
    await invoke("update_config", { 
        newConfig: {
            model_folder: modelFolder.value,
            model_path: modelPath.value,
            prefer_mmap: preferMmap.value,
            context_size: contextSize.value,
            use_gpu: useGpu.value,
            gpu_layers: gpuLayers.value,
        }
    });

    loading.value = false;

    emit("saved");
}

function close() {
    emit("close");
}
</script>

<style scoped>
fieldset {
    display: flex;
    margin-inline-start: 0px;
    margin-inline-end: 0px;
    padding-block-start: 0em;
    padding-inline-start: 0em;
    padding-inline-end: 0em;
    padding-block-end: 0em;
    min-inline-size: min-content;
    border-width: 0px;
    border-color: transparent;
}

.dialog-mask {
    position: fixed;
    height: 100%;
    width: 100%;
    left: 0px;
    top: 0px;
    display: flex;
    justify-content: center;
    align-items: center;
    pointer-events: auto;
    z-index: 1101;
    background-color: rgba(0, 0, 0, 0.4);
    transition-duration: 0.2s;
    overflow: hidden;
}

.dialog {
    display: flex;
    flex-direction: column;
    pointer-events: auto;
    width: 80%;
    margin: 0;
    border-radius: 12px;
    background-color: #f6f6f6;
    overflow-y: auto;
}

.dialog-header {
    border-top-right-radius: 12px;
    border-top-left-radius: 12px;
    border-bottom: 0 none;
    padding: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-shrink: 0;
}

.dialog-title {
    font-weight: 600;
    font-size: 1.25rem;
}

.dialog-close {
    display: flex;
    align-items: center;
}

.close-button {
    background-color: transparent;
    box-shadow: none;
}

.dialog-body {
    border-bottom-right-radius: 12px;
    border-bottom-left-radius: 12px;
    padding: 0 1.5rem 1.5rem 1.5rem;
    overflow-y: auto;
}

.dialog-row {
    display: flex;
    align-items: center;
}

.dialog-row > p {
    margin-right: 15px;
    min-width: 8em;
}

.dialog-row > input[type=text] {
    flex-grow: 1;
    max-width: 100%;
}

.dialog-row > input {
    max-width: 5em;
}

.dialog-footer {
    display: flex;
    flex-direction: row-reverse;
    border-bottom-right-radius: 12px;
    border-bottom-left-radius: 12px;
    padding: 0 1.5rem 1.5rem 1.5rem;
}

.browse-button {
    margin-left: 5px;
}

.cancel-button {
    margin-right: 5px;
    background-color: transparent;
    box-shadow: none;
}

@media (prefers-color-scheme: dark) {
  .dialog {
    background-color: #2f2f2f;
  }
}
</style>

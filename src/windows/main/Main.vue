<script setup lang="ts">
/* Tauri */
import { invoke } from '@tauri-apps/api';
// import { appWindow } from '@tauri-apps/api/window'
/* Vue */
import { ref } from 'vue';
/* Components */
import Titlebar from "@/components/Titlebar.vue"

const testres = ref("");
async function testsend() {
    testres.value = await invoke('mycustomcommand', { asd: "asdf" });
}

const windowName = "Pomni"
// appWindow.setTitle(windowName)

</script>

<template>
    <Titlebar>{{ windowName }}</Titlebar>
    <main>
        <button @click="testsend">
            communicate with rust
        </button>
        <div class="">
            {{ testres }}
        </div>
    </main>
</template>

<style scoped>
main {
    padding: 5%;
    display: flex;
    flex-direction: column;
}
</style>

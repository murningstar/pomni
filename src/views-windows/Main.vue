<script setup lang="ts">
/* Tauri */
import { invoke } from '@tauri-apps/api';
/* Vue */
import { ref } from 'vue';
/* Components */
import Titlebar from "@/components/Titlebar.vue"

const res = ref("");
async function send() {
    res.value = await invoke('mycustomcommand', { asd: "asdf" });
}
</script>

<template>
    <Titlebar>Pomni</Titlebar>
    <main>
        <button @click="send">
            communicate with rust
        </button>
        <div class="">
            {{ res }}
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

<script setup lang="ts">
/* Tauri */
import { invoke } from '@tauri-apps/api';
/* Vue */
import { ref } from 'vue';
/* Components */
import Window from "@/components/Window.vue"

const res = ref("");
async function send() {
    res.value = await invoke('mycustomcommand', { asd: "asdf" });
}
</script>

<template>
    <Window>
        <template #title>
            Pomni
        </template>
        <template #default>
            <main>
                <button @click="send">
                    communicate with rust
                </button>
                <div class="">
                    {{ res }}
                </div>
            </main>
        </template>
    </Window>
</template>

<style scoped>
main {
    padding: 5%;
    display: flex;
    flex-direction: column;
}

</style>

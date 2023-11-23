<script setup lang="ts">
import Titlebar from "@/components/Titlebar.vue"

</script>

<template>
    <Titlebar>
        <slot name="title"></slot>
    </Titlebar>
    <slot name="default"></slot>
</template>

<style scoped></style>
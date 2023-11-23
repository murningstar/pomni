<script setup lang="ts">
// Window -> Titlebar
import { appWindow } from '@tauri-apps/api/window'
</script>

<template>
    <div data-tauri-drag-region class="titlebar">

        <div class="signature">
            <div class="signature__icon">

            </div>
            <h1 class="signature__h1">
                <slot></slot>
            </h1>
        </div>

        <div class="actions">
            <div name="minimize" class="actions__button"
                @click="appWindow.minimize()">
                <img class="actions__icon"
                    src="https://api.iconify.design/mdi:window-minimize.svg"
                    alt="minimize" />
            </div>
            <div name="maximize" class="actions__button"
                @click="appWindow.toggleMaximize()">
                <img class="actions__icon"
                    src="https://api.iconify.design/mdi:window-maximize.svg"
                    alt="maximize" />
            </div>
            <div name="close" class="actions__button" @click="appWindow.close()">
                <img class="actions__icon"
                    src="https://api.iconify.design/mdi:close.svg" alt="close" />
            </div>
        </div>

    </div>
</template>

<style scoped lang="scss">
.titlebar {
    display: flex;
    height: $Titlebar_height;
    background: rgb(79, 172, 254);
    background: linear-gradient(90deg, rgba(79, 172, 254, 1) 0%, rgba(0, 242, 254, 1) 50%);
    user-select: none;
    justify-content: space-between;
    align-items: center;
}

.signature {
    display: flex;
    align-items: center;
    margin-left: 0.5rem;
    height: 100%;
    pointer-events: none;

    &__icon {
        width: 16px;
        height: 16px;
        background-color: #ffcc00;
        box-shadow: 1px 1px 0 0 black;

    }

    &__h1 {
        margin-left: 15px;
        font-size: 1rem;
        color: rgb(0, 0, 0);
        display: flex;
        font-family: 'Gill Sans', 'Gill Sans MT', Calibri, 'Trebuchet MS', sans-serif;
    }
}

.actions {
    display: flex;

    &__button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        height: $Titlebar_height;
        width: 30px;

        &:hover {
            background-color: rgba(0, 107, 143, 0.515);
        }
    }
}</style>
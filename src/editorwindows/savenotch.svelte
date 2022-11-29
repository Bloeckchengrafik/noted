<script lang="ts">
    import { Circle, Pulse, Stretch, SyncLoader } from "svelte-loading-spinners";

    export let current_save_state: number;
</script>

<div class={"notch " + (current_save_state > 0 ? "open" : "")}>
    {#if current_save_state == 1}
        <div class="spacing">
            <Circle color="var(--color-text)" size="12" unit="px" />
        </div>
    {:else}
        ✅
    {/if}
</div>

<style lang="sass">
    .notch
        position: fixed
        height: 3em
        padding: 0.7em
        right: 1em
        bottom: -2em
        border-radius: 1em
        background: var(--color-primary)
        z-index: 999
        transition: transform 0.2s ease-in-out
        transform: translateY(200%)

        &.open
            transform: translateY(0)

    .spacing
        margin-top: 0.2em
        width: 1.42em
        display: flex
        justify-content: center
        align-items: center
</style>

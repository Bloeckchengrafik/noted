<script lang="ts">
    import { afterInactivity, debounce, throttle } from "../utils/timing";
    import { onMount, onDestroy } from "svelte";
    import { update_await_block_branch } from "svelte/internal";
    import { get_contents, save_contents } from "../utils/fileops";
    import Style from "./editor.module.css";
    import Savenotch from "./savenotch.svelte";

    export let fqpn: string;

    let contentRef: HTMLPreElement;

    let saveState = 0;
    let initialized = false

    const load = async () => {
        contentRef.innerText =  await get_contents(fqpn) as string;
        initialized = true;
    };

    const onKeydown = async (e: KeyboardEvent) => {
        if (e.key === "s" && e.ctrlKey) {
            e.preventDefault();
            await save();
        }
    };

    onMount(() => {
        load();
        document.addEventListener("keydown", onKeydown);
    });

    onDestroy(async () => {
        if (initialized) await save_contents(fqpn, contentRef.innerText);
        document.removeEventListener("keydown", onKeydown);
    });

    const save = async () => {
        saveState = 1;

        await save_contents(fqpn, contentRef.innerText);
        await new Promise((resolve) => setTimeout(resolve, 2000));
        saveState = 2;
        await new Promise((resolve) => setTimeout(resolve, 1000));
        saveState = 0;
    };

    const autosave = afterInactivity(save, 5000);
</script>

<div class={Style.container}>
    <pre contenteditable="true" class={Style.editorwindow} on:keydown={autosave} bind:this={contentRef}></pre>
</div>

<Savenotch current_save_state={saveState} />

<style lang="sass">
    .pre
        // Enable line wrapping
        white-space: pre-wrap
</style>

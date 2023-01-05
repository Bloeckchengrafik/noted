<script lang="ts">
    import {afterInactivity} from "../utils/timing";
    import {onMount, onDestroy} from "svelte";
    import {get_contents, save_contents} from "../utils/fileops";
    import SaveIcon from "../lib/SaveIcon.svelte";
    import {changeSizeKeybindListener} from "../utils/editor";

    export let fqpn: string;
    export let contents: string;

    let saveState = 0;
    let initialized = false

    const load = async () => {
        contents = await get_contents(fqpn) as string;
        initialized = true;
    };

    const onKeydown = async (e: KeyboardEvent) => {
        if (e.key === "s" && e.ctrlKey) {
            e.preventDefault();
            await save();
        }
    };

    onMount(async () => {
        await load();
        document.addEventListener("keydown", onKeydown);
        document.addEventListener("keydown", changeSizeKeybindListener)
    });

    onDestroy(async () => {
        if (initialized) await save_contents(fqpn, contents);
        document.removeEventListener("keydown", onKeydown);
        document.removeEventListener("keydown", changeSizeKeybindListener)
    });

    export const save = async () => {
        saveState = 1;

        await save_contents(fqpn, contents);
        await new Promise((resolve) => setTimeout(resolve, 1000));
        saveState = 2;
        await new Promise((resolve) => setTimeout(resolve, 500));
        saveState = 0;
    };

    export const autosave = afterInactivity(save, 5000);
</script>

<slot />

<SaveIcon current_save_state={saveState}/>

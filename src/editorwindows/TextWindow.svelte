<script lang="ts">
    import {afterInactivity} from "../utils/timing";
    import {onMount, onDestroy} from "svelte";
    import {get_contents, save_contents} from "../utils/fileops";
    import Style from "./editor.module.css";
    import SaveIcon from "../lib/SaveIcon.svelte";
    import {changeSizeKeybindListener} from "../utils/editor";

    export let fqpn: string;

    let contentRef;

    let saveState = 0;
    let initialized = false

    let empty = false;
    let forceInvisible = false;
    let currentMessage = 0
    let centerTextRef

    const centerMessages = [
        "Just start typing...",
        "There is no shame in starting with a blank page.",
        "Just begin...",
        "This is where the story begins..",
        "In a hole in the ground there lived a hobbit...",
    ]

    let intervalUpdateEmptyMessage;

    const load = async () => {
        contentRef.innerText = await get_contents(fqpn) as string;
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

        empty = contentRef.innerText === "" || contentRef.innerText === "\n";

        intervalUpdateEmptyMessage = setInterval(async () => {
            currentMessage++;

            if (currentMessage >= centerMessages.length) {
                currentMessage = 0
            }

            if (centerTextRef) {
                forceInvisible = true
                await new Promise(resolve => setTimeout(resolve, 500))
                centerTextRef.innerText = centerMessages[currentMessage]
                await new Promise(resolve => setTimeout(resolve, 500))
                forceInvisible = false
            }

        }, 10000)
    });

    onDestroy(async () => {
        if (initialized) await save_contents(fqpn, contentRef.innerText);
        document.removeEventListener("keydown", onKeydown);
        document.removeEventListener("keydown", changeSizeKeybindListener)

        clearInterval(intervalUpdateEmptyMessage)
    });

    const save = async () => {
        saveState = 1;

        await save_contents(fqpn, contentRef.innerText);
        await new Promise((resolve) => setTimeout(resolve, 1000));
        saveState = 2;
        await new Promise((resolve) => setTimeout(resolve, 500));
        saveState = 0;
    };

    const autosave = afterInactivity(save, 5000);

    const beginSave = (event) => {
        empty = event.target.innerText === "" || event.target.innerText === "\n";

        autosave()
    }
</script>

<div class={Style.container}>
    <!-- Text field with disabled spellchecking -->
    <pre bind:this={contentRef} class={Style.editorwindow + " pre"} contenteditable="true" on:keyup={beginSave}
        spellcheck="false"
    ></pre>

    <div class={"behind-text " + (empty ? " " : "invisible ") + (forceInvisible ? "force-invisible" : "")} bind:this={centerTextRef}>
        Just start typing...
    </div>
</div>

<SaveIcon current_save_state={saveState}/>

<style lang="sass">
  .pre
    white-space: normal
    width: 100%
    max-width: 800px!important
    margin: 0 auto
    line-height: 1.2em
    padding-top: 4em
    z-index: 1

  .behind-text
    position: absolute
    top: 0
    left: 0
    right: 0
    bottom: 0
    display: flex
    align-items: center
    justify-content: center
    color: var(--color-background-secondary)
    font-size: 2em
    font-style: italic
    pointer-events: none
    z-index: -1
    opacity: 1
    font-family: "Victor Mono", monospace
    font-weight: lighter
    letter-spacing: -2px
    transition: opacity 0.2s ease-in-out

  .invisible
    opacity: 0

  .force-invisible
    opacity: 0

@media screen and (max-width: 800px)
  .pre
    max-width: 100% !important

</style>

<script lang="ts">
    import { hasFinderOpen } from "../stores";
    import { onDestroy, onMount } from "svelte";

    let inputRef: HTMLInputElement;
    let inputValue: string = "";

    const onKeypress = (e: KeyboardEvent) => {
        if (e.key === "Escape") {
            e.preventDefault();
            e.stopPropagation();

            hasFinderOpen.set(false);
        }
    };

    onMount(() => {
        document.addEventListener("keydown", onKeypress);

        inputRef.focus();
    });

    onDestroy(() => {
        document.removeEventListener("keydown", onKeypress);
    });
</script>

<div class="finder">
    <div class="finder-window">
        <h1>Quick Finder</h1>
        <input
            type="text"
            placeholder="Search..."
            bind:this={inputRef}
            bind:value={inputValue}
        />
        <ul>
            <li>Nr. 1: {inputValue}</li>
            <li>Nr. 2</li>
            <li>Nr. 3</li>
            <li>Nr. 4</li>
        </ul>
    </div>
</div>

<style lang="sass">
.finder
  position: absolute
  top: 0
  left: 0
  width: 100%
  height: 100%
  background-color: rgba(0, 0, 0, 0.5)
  z-index: 99999

.finder-window
    position: absolute
    top: 50%
    left: 50%
    transform: translate(-50%, -50%)
    width: 600px
    height: 300px
    background-color: var(--color-background-secondary)
    border-radius: 5px
    padding: 20px
    display: flex
    flex-direction: column
    align-items: center

    h1
        margin: 0
        font-size: 1.5rem
        text-align: center

    input
        width: 90%
        height: 40px
        border: 1px solid var(--color-border)
        background: var(--color-background)
        border-radius: 5px
        padding: 0 10px
        margin-top: 20px
        margin-bottom: 20px
        font-size: 1rem
        color: var(--color-text)

        &:focus
            outline: none
            border-color: var(--color-primary)

    ul
        width: 100%
</style>

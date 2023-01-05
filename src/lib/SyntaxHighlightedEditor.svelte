<script lang="ts">
    import "../syntax/md.css"
    import {highlightMD} from "../syntax/md";

    const langs = {
        "md": highlightMD,
    }

    export let lang = "md";
    let langFn
    $: langFn = langs[lang];

    export let codeUnhighlighted = ""
    let codeHighlighted = ""
    let inputField

    export let onUpdate = (_) => {}

    function getLineNumber() {
        if (!inputField) {
            return -1
        }

        for (let domObj of document.querySelectorAll(":focus, :active")){
            if (domObj === inputField) {
                return inputField.value.substring(0, inputField.selectionStart).split("\n").length - 1
            }
        }

        return -1;
    }

    function highlightCode(code: string): string {
        if (onUpdate) {
            onUpdate(code)
        }

        return langFn(code, getLineNumber())
    }

    document.onselectionchange = function () {
        codeHighlighted = highlightCode(codeUnhighlighted)
    }

    $: codeHighlighted = highlightCode(codeUnhighlighted)
</script>

<div class="container editor">
    <!-- This is the editable code in the background -->
    <pre><textarea bind:this={inputField} bind:value={codeUnhighlighted} class="editor-text"></textarea></pre>
    <!-- This is the visual code in the foreground -->
    <pre class="overlay"><code class="overlay editor-text">{@html codeHighlighted}</code></pre>
</div>

{#if codeHighlighted === "" || codeHighlighted === "\n"}
    <div class="placeholder">Type some code...</div>
{/if}

<style lang="sass">
  .editor-text
    line-height: 1.2em !important
    font-family: monospace !important
    font-size: 1.2em !important

  .container
    position: relative
    white-space: normal
    height: 50%
    width: 100%
    max-width: 800px !important
    margin: 0 auto
    padding-top: 4em
    z-index: 1

    pre
      position: absolute
      top: 0
      left: 0
      width: 100%
      height: 100%

      code
        display: block
        width: 100%
        height: 100%
        z-index: 1


    textarea
      color: transparent
      background: transparent
      border: none
      resize: none
      margin: 0
      padding: 0
      width: 100%
      height: 100%
      caret-color: whitesmoke
      z-index: -1

      &:focus
        outline: none

  .overlay
    pointer-events: none

  .placeholder
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
    transition: opacity 0.2s ease-in-out
</style>
<script lang="ts">
    import SyntaxHighlightedEditor from "$lib/SyntaxHighlightedEditor.svelte";
    import SaveableEditorWrapper from "$lib/SaveableEditorWrapper.svelte";

    export let fqpn;
    let code;
    let autosave
</script>


<SaveableEditorWrapper
    fqpn={fqpn}
    bind:contents={code}
    bind:autosave
>
    <SyntaxHighlightedEditor
            bind:codeUnhighlighted={code}
            lang="md"
            onUpdate={autosave}
    ></SyntaxHighlightedEditor>
</SaveableEditorWrapper>

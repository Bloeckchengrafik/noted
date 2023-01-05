<script>
    import { get_type } from "../utils/fileextensions";
    import EmptyWindow from "../editorwindows/EmptyWindow.svelte";
    import TextWindow from "../editorwindows/TextWindow.svelte";
    import { currentTab } from "../stores";
    import MDWindow from "../editorwindows/MDWindow.svelte";
</script>
{#key $currentTab}
    {#if $currentTab === ""}
        <EmptyWindow />
    {:else if get_type($currentTab) === "text"}
        <TextWindow fqpn={$currentTab} />
    {:else if get_type($currentTab) === "md"}
        <MDWindow fqpn={$currentTab} />
    {:else}
        <EmptyWindow />
    {/if}
{/key}

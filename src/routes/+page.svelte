<script>
    import EmptyWindow from "../editorwindows/EmptyWindow.svelte";
    import TextWindow from "../editorwindows/TextWindow.svelte";
    import { currentTab } from "../stores";
</script>
{#key $currentTab}
    {#if $currentTab === ""}
        <EmptyWindow />
    {:else}
        <TextWindow fqpn={$currentTab} />
    {/if}
{/key}

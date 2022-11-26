<script lang="ts">
    import {Folder, File, CaretRight} from "phosphor-svelte";

    export type FileNode = {
        name: string;
        node_type: "File" | "Directory";
        children: FileNode[];
    }

    export let node: FileNode
</script>

<div>
    {#if node.node_type === "Directory"}
        <CaretRight/>
        <Folder/>
    {:else}
        &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<File/>
    {/if}

    {node.name}
    {#if node.node_type === "Directory"}
        <ul>
            {#each node.children as child}
                <li>
                    {#if !child.name.startsWith(".")}
                        <svelte:self node={child}/>
                    {/if}
                </li>
            {/each}
        </ul>
    {/if}
</div>

<style lang="sass">
  ul
    list-style: none
    padding-left: 0
    margin-left: 1rem

  .has-left-border
    border-left: 1px solid var(--color-border)
</style>
<script lang="ts">
    import {reloadFileTree, sidebarOpen} from "../stores";
    import {invoke} from "@tauri-apps/api/tauri";
    import FileTree from "./FileTree.svelte";
    import {onMount, onDestroy} from "svelte";

    export type FileNode = {
        name: string;
        node_type: "File" | "Directory";
        children: FileNode[];
    }

    let file_tree: FileNode | null = null;

    const loadFileTree = async () => {
        let tree: { root: FileNode } = await invoke("get_filetree")
        tree.root.name = "Vault"
        file_tree = tree.root;
    }

    loadFileTree()

    let interval: NodeJS.Timer = null;

    onMount(() => {
        interval = setInterval(loadFileTree, 4000)
    })

    onDestroy(() => {
        clearInterval(interval)
    })

    $: reloadFileTree.subscribe(() => {
        loadFileTree().then(() => {
            console.log("Reloaded file tree from interrupt")
        })
    })

</script>

<div class="sidenav sidenav-transition" style="width: {$sidebarOpen?'300':'0'}px">
    <div class="mask">
        <div style="height: 50px"></div>

        {#key file_tree}
            {#if file_tree}
                <ul class="file-nav">
                    <FileTree node={file_tree}/>
                </ul>
            {/if}
        {/key}

    </div>
</div>

<div class="sidenav-transition" style="margin-left: {$sidebarOpen?'300':'0'}px;height: 100vh;position:relative">
    <div style="height: 50px"></div>
    <slot/>
</div>

<style lang="sass">
  .sidenav
    width: 0
    position: fixed
    z-index: 1
    top: 0
    left: 0
    bottom: 0
    background: var(--color-background-tertiary)
    overflow-x: hidden

  .sidenav h4
    margin: 0
    padding: 10px 0 0 10px
    font-size: 20px
    color: var(--color-text-primary)

  .sidenav-transition
    transition: width 0.5s, margin-left 0.5s

  .mask
    width: 300px

  .file-nav
    list-style-type: none
    margin: 1em
    padding: 0
    overflow: hidden
    background-color: var(--color-background-tertiary)
</style>
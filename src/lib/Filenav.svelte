<script lang="ts">
    import {sidebarOpen} from "../stores";
    import {invoke} from "@tauri-apps/api/tauri";
    import FileTree from "./FileTree.svelte";

    export type FileNode = {
        name: string;
        node_type: "File" | "Directory";
        children: FileNode[];
    }

    let filetree: FileNode | null = null;

    const loadFileTree = async () => {
        let tree: { root: FileNode } = await invoke("get_filetree")

        filetree = tree.root;
        filetree.name = "Vault";

        console.log("Reloaded filetree")
    }

    loadFileTree()
    setInterval(loadFileTree, 2000)

</script>

<div class="sidenav sidenav-transition" style="width: {$sidebarOpen?'300':'0'}px">
    <div class="mask">
        <div style="height: 50px"></div>
        <h4>File Manager</h4>
        <!--
         A recursive component that renders the file tree that is open and closeable
         -->
        {#if (filetree)}
            <ul class="file-nav">
                <FileTree node={filetree}/>
            </ul>
        {/if}
    </div>
</div>

<div class="sidenav-transition" style="margin-left: {$sidebarOpen?'300':'0'}px">
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
    transition: width 0.5s

  .mask
    width: 300px

  .file-nav
    list-style-type: none
    margin: 1em
    padding: 0
    overflow: hidden
    background-color: var(--color-background-tertiary)
</style>
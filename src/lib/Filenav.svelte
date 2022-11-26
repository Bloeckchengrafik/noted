<script lang="ts">
    import {sidebarOpen} from "../stores";
    import {invoke} from "@tauri-apps/api/tauri";

    export type FileNode = {
        name: string;
        node_type: "File" | "Directory";
        children: FileNode[];
    }

    export type FileTree = {
        root: FileNode;
    }

    let filetree: FileTree | null;

    invoke("get_filetree").then((data: FileTree) => {
        filetree = data;

        console.log(filetree);
    });
</script>

<div class="sidenav sidenav-transition" style="width: {$sidebarOpen?'300':'0'}px">
    <div style="height: 50px"></div>
    <h4>File Manager</h4>
    {#if (filetree)}
        <ul>
            {#each filetree.root.children as child}
                <li>
                    {child.name}
                    {#if (child.node_type === "Directory")}
                        <ul>
                            {#each child.children as grandchild}
                                <li>
                                    {grandchild.name}
                                </li>
                            {/each}
                        </ul>
                    {/if}
                </li>
            {/each}
        </ul>
    {/if}
</div>

<div class="sidenav-transition" style="margin-left: {$sidebarOpen?'300':'0'}px">
    <slot />
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

</style>
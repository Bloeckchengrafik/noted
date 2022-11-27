<script lang="ts">
    import type {FileCtxMenuPayload} from "../../stores";
    import {FilePlus, Tabs, FolderOpen, AppWindow, Copy, NotePencil, Trash} from "phosphor-svelte";
    import {question, toast} from "../../utils/alerts.js";
    import {reloadFileTree} from "../../stores.js";
    import {invoke} from "@tauri-apps/api/tauri";
    import {copy} from "../../utils/copy.js";

    /*
     * Show the file context menu
     * It contains the following options:
     * - Open
     * - Open in new tab
     * - - -
     * - Show in file explorer
     * - Open in default application
     * - Copy full path
     * - - -
     * - Rename
     * - Delete
     */

    export let ctxMenu: FileCtxMenuPayload
    export let done: () => void
</script>

<div on:click={async () => {
    done()

    await toast("open file")
}}><FilePlus />Open</div>

<div on:click={async () => {
    console.log("open file in new tab")
    done()
}}><Tabs /> Open in new tab</div>
<hr />

<div on:click={async () => {
    console.log("show in file explorer")
    done()
}}><FolderOpen /> Show in file explorer</div>

<div on:click={async () => {
    console.log("open in default application")
    done()
}}><AppWindow /> Open in default application</div>

<div on:click={async () => {
    done()

    let basePath = await invoke("get_base_path")
    let fullPath = basePath + ctxMenu.fqpn
    copy(fullPath)
    await toast("Copied the full path to the system clipboard")
}}><Copy /> Copy full path</div>
<hr />

<div on:click={async () => {
    console.log("rename")
    done()

    let data = await question("Rename", "Enter new name", ctxMenu.filename)

    if (!data.value) return

    console.log("new name", data.value)

    $reloadFileTree = !$reloadFileTree

    if (!await invoke("rename", {fqpn: ctxMenu.fqpn.substring(1), newName: data.value})) {
        await toast("Failed to rename file", "", "error")
    } else {
        await toast("File renamed")
    }
}}><NotePencil /> Rename</div>

<div on:click={async () => {
    console.log("delete")
    done()
}}><Trash /> Delete</div>

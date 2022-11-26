<script lang="ts">
    import type {FileCtxMenuPayload} from "../../stores";
    import {FilePlus, Tabs, FolderOpen, AppWindow, Copy, NotePencil, Trash} from "phosphor-svelte";
    import {question, toast} from "../../utils/alerts.js";
    import {reloadFileTreeTrigger} from "../../stores.js";
    import {invoke} from "@tauri-apps/api/tauri";

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
    console.log("copy full path")
    done()
}}><Copy /> Copy full path</div>
<hr />

<div on:click={async () => {
    console.log("rename")
    done()

    let data = await question("Rename", "Enter new name", ctxMenu.filename)

    if (!data.value) return

    console.log("new name", data.value)

    if (!await invoke("rename", {fqpn: ctxMenu.fqpn.substring(1), newName: data.value})) {
        await toast("Failed to rename file", "", "error")
    } else {
        await toast("File renamed")
    }

    $reloadFileTreeTrigger()
}}><NotePencil /> Rename</div>

<div on:click={async () => {
    console.log("delete")
    done()
}}><Trash /> Delete</div>

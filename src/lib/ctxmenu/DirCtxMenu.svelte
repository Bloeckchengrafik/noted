<script lang="ts">
    import type {FileCtxMenuPayload} from "../../stores";
    import {FilePlus, FolderPlus, FolderOpen, TerminalWindow, Copy, NotePencil, Trash} from "phosphor-svelte";
    import {question, toast} from "../../utils/alerts.js";
    import {reloadFileTree} from "../../stores.js";
    import {invoke} from "@tauri-apps/api/tauri";
    import {copy} from "../../utils/copy.js";

    /*
     * - New file
     * - New directory
     * - - -
     * - Show in file explorer
     * - Open in terminal
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

    const answer = await question("How should the file be named?")
    if (!answer.value) return

    const path = ctxMenu.fqpn.substring(1) + answer.value
    await invoke("create_file", {fqpn: path})
    $reloadFileTree = !$reloadFileTree

    await toast("File created")
}}><FilePlus />New File</div>

<div on:click={async () => {
    done()

    const answer = await question("How should the directory be named?")

    if (!answer.value) return

    const path = ctxMenu.fqpn.substring(1) + answer.value
    await invoke("create_dir", {fqpn: path})
    $reloadFileTree = !$reloadFileTree

    await toast("Directory created")
}}><FolderPlus /> New Directory</div>
<hr />

<div on:click={async () => {
    console.log("show in file explorer")
    done()

    if (!await invoke("open_in_explorer", {fqpn: ctxMenu.fqpn.substring(1)})) {
        await toast("Failed to open", "", "error")
    } else {
        await toast("File opened")
    }
}}><FolderOpen /> Show in file explorer</div>

<div on:click={async () => {
    console.log("open in terminal")
    done()

    if (!await invoke("open_in_default_app", {fqpn: ctxMenu.fqpn.substring(1)})) {
        await toast("Failed to open", "", "error")
    } else {
        await toast("File opened")
    }
}}><TerminalWindow /> Open in Terminal</div>

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

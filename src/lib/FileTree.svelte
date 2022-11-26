<script lang="ts">
    import {
        Folder,
        File,
        CaretRight,
        FileDoc,
        FileAudio,
        FileCloud,
        FileCsv,
        FileImage, FileLock, FileVideo, FileCode
    } from "phosphor-svelte";
    import {settings, currentCtxMenuSettings} from "../stores.js";

    export type FileNode = {
        name: string;
        node_type: "File" | "Directory";
        children: FileNode[];
    }

    export let node: FileNode = {name: "", node_type: "Directory", children: []}
    export let fqpn: string = "/" // fully qualified path name

    let fileChildren: FileNode[] = []
    let dirChildren: FileNode[] = []

    for (let child of node.children) {
        if (child.name.startsWith(".")) {
            continue
        }

        if (child.node_type == "File") {
            fileChildren.push(child)
        } else {
            dirChildren.push(child)
        }
    }

    fileChildren.sort((a, b) => a.name.localeCompare(b.name))
    dirChildren.sort((a, b) => a.name.localeCompare(b.name))

    let CurrentFileIcon = File

    const fileExtension = node.name.split(".").pop()

    const extensions = {
        "md": FileDoc, // markdown
        "mdx": FileDoc, // markdown x
        "mp3": FileAudio, // mp3 audio
        "wav": FileAudio, // wav audio
        "ogg": FileAudio, // ogg audio
        "sym": FileCloud, // cloud link
        "csv": FileCsv, // csv spreadsheet
        "png": FileImage, // png image
        "jpg": FileImage, // jpg image
        "jpeg": FileImage, // jpeg image, same as jpg
        "gif": FileImage, // gif image
        "svg": FileImage, // svg image
        "webp": FileImage, // webp image
        "lock": FileLock, // lock file
        "mp4": FileVideo, // mp4 video
        "mov": FileVideo, // mov video
        "mkv": FileVideo, // mkv video
        "avi": FileVideo, // avi video
        "flv": FileVideo, // flv video
        "wmv": FileVideo, // wmv video
        "webm": FileVideo, // webm video
        "js": FileCode, // javascript
        "ts": FileCode, // typescript
        "jsx": FileCode, // jsx
        "tsx": FileCode, // tsx
        "py": FileCode, // python
        "rb": FileCode, // ruby
        "go": FileCode, // go
        "java": FileCode, // java
        "c": FileCode, // c
        "cpp": FileCode, // c++
        "h": FileCode, // c header
        "hpp": FileCode, // c++ header
        "cs": FileCode, // c#
        "css": FileCode, // css
        "html": FileCode, // html
        "htm": FileCode, // html
        "json": FileCode, // json
        "toml": FileCode, // toml
        "yaml": FileCode, // yaml
        "yml": FileCode, // yaml
        "xml": FileCode, // xml
        "sh": FileCode, // shell script
        "bat": FileCode, // batch script
        "cmd": FileCode, // command script
        "ps1": FileCode, // powershell script
        "psm1": FileCode, // powershell module
        "psd1": FileCode, // powershell data
        "bash": FileCode, // bash script
        "zsh": FileCode, // zsh script
        "fish": FileCode, // fish script
        "csh": FileCode, // csh script
        "jl": FileCode, // julia
        "kt": FileCode, // kotlin
        "kts": FileCode, // kotlin script
        "clj": FileCode, // clojure
        "ex": FileCode, // elixir
        "exs": FileCode, // elixir script
        "eex": FileCode, // elixir embedded
        "lfe": FileCode, // lisp
        "lisp": FileCode, // lisp
        "lsp": FileCode, // lisp
        "lua": FileCode, // lua
        "m": FileCode, // matlab
        "mat": FileCode, // matlab
        "mjs": FileCode, // javascript module
        "php": FileCode, // php
        "pl": FileCode, // perl
        "pm": FileCode, // perl module
        "psgi": FileCode, // perl script
        "r": FileCode, // r
        "rs": FileCode, // rust
        "rsx": FileCode, // rust embedded
        "scala": FileCode, // scala
        "scm": FileCode, // scheme
        "sql": FileCode, // sql
        "swift": FileCode, // swift
        "vb": FileCode, // visual basic
        "vbs": FileCode, // visual basic script
        "vim": FileCode, // vim script
        "vimrc": FileCode, // vim config
        "tex": FileCode, // latex
        "bib": FileCode, // bibtex
        "cls": FileCode, // latex class
        "sty": FileCode, // latex style
        "dtx": FileCode, // latex document
    }

    if (node.node_type == "Directory") {
        CurrentFileIcon = Folder
    } else {
        if (extensions[fileExtension]) {
            CurrentFileIcon = extensions[fileExtension]
        }
    }

    const onClickFile = (opened_dirs: { includes: (val: string) => boolean }) => {
        console.log("file clicked")
    }

    const onClickDir = (opened_dirs: { includes: (val: string) => boolean }) => {
        console.log("dir clicked")
        if (opened_dirs.includes(fqpn)) {
            settings.update(s => {
                s.opened_dirs = s.opened_dirs.filter(d => d !== fqpn)
                return s
            })
        } else {
            settings.update(s => {
                s.opened_dirs = [...s.opened_dirs, fqpn]
                return s
            })
        }
    }

    /**
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
     *
     * @param opened_dirs the list of opened directories
     * @param e the event that triggered the context menu
     */
    function onContextFile(opened_dirs: { includes: (val: string) => boolean }, e: MouseEvent) {
        console.log("file context")

        let fileContextMenuX = e.clientX-10
        let fileContextMenuY = e.clientY-10

        // Make sure the context menu is not off the screen
        if (fileContextMenuX + 200 > window.innerWidth) {
            fileContextMenuX = window.innerWidth - 200
            console.warn("Context menu is off the screen, moving it to the left")
        }

        if (fileContextMenuY + 300 > window.innerHeight) {
            fileContextMenuY = window.innerHeight - 300
            console.warn("Context menu is off the screen, moving it to the top")
        }

        // There is another 50px of padding at the top of the screen
        if (fileContextMenuY < 50) {
            fileContextMenuY = 50
            console.warn("Context menu is off the screen, moving it to the bottom")
        }

        currentCtxMenuSettings.update(s => {
            s.x = fileContextMenuX
            s.y = fileContextMenuY
            s.visible = true
            s.payload = {
                type: "file",
                fqpn: fqpn,
                filename: node.name,
            }
            return s
        })
    }

    /**
     * Show the directory context menu
     * It contains the following options:
     * - New file
     * - New directory
     * - - -
     * - Show in file explorer
     * - Open in terminal
     * - Copy full path
     * - - -
     * - Rename
     * - Move
     * - Delete
     *
     * @param opened_dirs the list of opened directories
     * @param event the event that triggered the context menu
     */
    function onContextDir(opened_dirs: { includes: (val: string) => boolean }, event: MouseEvent) {
        console.log("dir context")
    }

</script>

<div>
    <div class="clickable hoverable max-one-line"
         on:click={(event) => {
            event.stopPropagation()

            if (node.node_type === "File") {
                // noinspection JSCheckFunctionSignatures
                onClickFile($settings.opened_dirs)
            } else {
                // noinspection JSCheckFunctionSignatures
                onClickDir(($settings.opened_dirs))
            }
        }}
         on:contextmenu={(event) => {
            event.stopPropagation()
            event.preventDefault()

            if (node.node_type === "File") {
                // noinspection JSCheckFunctionSignatures
                onContextFile($settings.opened_dirs, event)
            } else {
                // noinspection JSCheckFunctionSignatures
                onContextDir($settings.opened_dirs, event)
            }
     }}>
        {#if node.node_type === "Directory"}
            <span class={"rotate-next " + ($settings.opened_dirs.includes(fqpn) ? "rotate" : "")}></span>
            <CaretRight/>
            <svelte:component this={CurrentFileIcon}/>
        {:else}
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<svelte:component this={CurrentFileIcon}/>
        {/if}

        {node.name}
    </div>
    {#if node.node_type === "Directory" && $settings.opened_dirs.includes(fqpn)}
        <ul>
            {#each dirChildren as child}
                <li>
                    <svelte:self node={child} fqpn={fqpn + child.name + "/"}/>
                </li>
            {/each}
        </ul>
        <ul class="has-left-border">
            {#each fileChildren as child}
                <li>
                    <svelte:self node={child} fqpn={fqpn + child.name}/>
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
    position: relative

  .has-left-border:after
    content: ""
    position: absolute
    top: 5px
    left: 10px
    bottom: 5px
    width: 1px
    background-color: var(--color-border)

  .clickable
    cursor: pointer
    -webkit-user-select: none
    -moz-user-select: none
    -ms-user-select: none
    user-select: none

  .rotate-next + :global(svg)
    transition: transform 0.1s ease-in-out

  .rotate + :global(svg)
    transform: rotate(90deg)

  .hoverable
    padding: 1px
    border-radius: 5px

    &:hover
      background-color: var(--color-background-secondary)

  .max-one-line
    max-width: 100%
    overflow: hidden
    text-overflow: ellipsis
    white-space: nowrap
    position: relative
</style>
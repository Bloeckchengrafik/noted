import {writable} from "svelte/store";
import type {Writable} from "svelte/store";

export type Settings = {
    file_tree_open: boolean;
    opened_dirs: string[];
    opened_files: string[];
    editor_font_size: number;
}

export type NullCtxMenuPayload = {
    type: "null";
}

export type FileCtxMenuPayload = {
    type: "file";
    fqpn: string;
    filename: string;
}

export type DirCtxMenuPayload = {
    type: "dir";
    fqpn: string;
    dirname: string;
}

export type CtxMenuPayload = FileCtxMenuPayload | DirCtxMenuPayload | NullCtxMenuPayload;

export type CtxMenu = {
    x: number;
    y: number;
    visible: boolean;
    payload: CtxMenuPayload;
}

export const sidebarOpen = writable(false);
export const settings: Writable<Settings> = writable({
    file_tree_open: true,
    opened_dirs: [],
    opened_files: [],
    editor_font_size: 14,
});

settings.subscribe((settings) => {
    document.body.style.setProperty("--editor-font-size", `${settings.editor_font_size}px`);
})

export const currentCtxMenuSettings: Writable<CtxMenu> = writable({
    x: 0,
    y: 0,
    visible: false,
    payload: { type: "null" }
})

export const reloadFileTree = writable(false);

export const hasLoaded = writable(false);
export const currentTab = writable("");

export const hasSettingsOpen = writable(false);
export const hasFinderOpen = writable(false);
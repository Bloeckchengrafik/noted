import {writable} from "svelte/store";
import type {Writable} from "svelte/store";

export type Settings = {
    file_tree_open: boolean;
    opened_dirs: string[];
}

export const sidebarOpen = writable(false);
export const settings: Writable<Settings> = writable({
    file_tree_open: true,
    opened_dirs: [],
});

export const openCtxMenus: Writable<(()=>void)[]> = writable([]);

export const hasLoaded = writable(false);
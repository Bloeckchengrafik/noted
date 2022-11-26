import {writable} from "svelte/store";
import type {Writable} from "svelte/store";

export type Settings = {
    file_tree_open: boolean;
}

export const sidebarOpen = writable(false);
export const settings: Writable<Settings> = writable({
    file_tree_open: true,
});

export const hasLoaded = writable(false);
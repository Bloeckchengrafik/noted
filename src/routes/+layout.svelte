<script lang="ts">
    import "../app.css";
    import Titlebar from "$lib/Titlebar.svelte";
    import FileNav from "$lib/FileNav.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import {currentTab, hasLoaded, settings, sidebarOpen} from "../stores";
    import type {Settings} from "../stores";
    import {onDestroy, onMount} from "svelte";
    import Preloader from "$lib/Preloader.svelte";
    import CtxMenu from "$lib/CtxMenu.svelte";
    import {currentCtxMenuSettings, hasFinderOpen, hasSettingsOpen} from "../stores.js";
    import SettingsPage from "$lib/SettingsPage.svelte";
    import {onKeydown} from "../utils/globalShortcuts";
    import QuickFinder from "$lib/QuickFinder.svelte";

    onMount(async () => {
        let settingsAnswer = (await invoke("get_settings")) as Settings;

        settings.set(settingsAnswer);
        console.log("Settings loaded: ", settingsAnswer);
        sidebarOpen.set(settingsAnswer.file_tree_open);
        if (settingsAnswer.opened_files.length > 0) {
            $currentTab = settingsAnswer.opened_files[0];
        }

        settings.subscribe((value) => {
            if (!hasLoaded) {
                console.log("Shouldn't be saving settings yet");
                return;
            }
            console.log("Saving settings: ", value);
            invoke("save_settings", {settings: value}).then((_) =>
                console.log("Settings saved!")
            );
        });

        sidebarOpen.subscribe((value) => {
            if (!hasLoaded) return;
            settings.update((settings) => {
                settings.file_tree_open = value;
                console.log("Settings updated: ", settings);
                return settings;
            });
        });

        await new Promise((resolve) => setTimeout(resolve, 1000));

        document.addEventListener("keydown", onKeydown)

        hasLoaded.set(true);
    });

    onDestroy(() => {
        document.removeEventListener("keydown", onKeydown)
    })
</script>

{#if !$hasLoaded}
    <Preloader/>
{:else}
    {#key currentCtxMenuSettings}
        <CtxMenu ctxMenu={$currentCtxMenuSettings}/>
    {/key}
    {#if $hasSettingsOpen}
        <SettingsPage />
    {/if}
    {#if $hasFinderOpen}
        <QuickFinder/>
    {/if}
    <Titlebar/>
    <FileNav>
        <slot/>
    </FileNav>
{/if}

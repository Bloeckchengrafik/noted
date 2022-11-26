<script lang="ts">
    import "../app.css";
    import Titlebar from "$lib/Titlebar.svelte";
    import LogRocket from 'logrocket';
    import Filenav from "$lib/Filenav.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import {hasLoaded, settings, sidebarOpen} from "../stores";
    import type {Settings} from "../stores";
    import {onMount} from "svelte";
    import Preloader from "$lib/Preloader.svelte";

    LogRocket.init('hgalrl/noted');

    invoke("current_user").then((user: string) => {
        LogRocket.identify(user);
    });

    onMount(async () => {
        let settingsAnswer = await invoke("get_settings") as Settings;

        settings.set(settingsAnswer);
        console.log("Settings loaded: ", settingsAnswer);
        sidebarOpen.set(settingsAnswer.file_tree_open);


        settings.subscribe((value) => {
            if (!hasLoaded) return;
            invoke("save_settings", {settings: value}).then(_ => console.log("Settings saved!"));
        })

        sidebarOpen.subscribe((value) => {
            if (!hasLoaded) return;
            settings.update(settings => {
                settings.file_tree_open = value;
                console.log("Settings updated: ", settings);
                return settings;
            });
        })

        await new Promise(resolve => setTimeout(resolve, 1000));

        hasLoaded.set(true);
    });
</script>

{#if !$hasLoaded}
    <Preloader />
{:else}
    <Titlebar/>
    <Filenav>
        <slot/>
    </Filenav>
{/if}

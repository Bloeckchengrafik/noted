<script lang="ts">
  import { Notebook, Folder, Gear } from "phosphor-svelte";
  import Tab from "$lib/Tab.svelte";
  import { sidebarOpen, settings, hasSettingsOpen } from "../stores.js";
</script>

<div class="titlebar">
  <div class="quick-actions">
    <div
      on:click={() => {
        $sidebarOpen = !$sidebarOpen;
        console.log("toggle sidebar");
      }}
    >
      <Folder />
    </div>
  </div>
  <div class="v-sep"></div>
  <div class="tabs">
    {#key settings}
      {#each $settings.opened_files as tab}
        <Tab {tab} />
      {/each}
    {/key}
  </div>
  <div class="v-sep"></div>
  <div class="quick-actions">
    <div>
      <Notebook />
    </div>
    <div on:click={() => {
      $hasSettingsOpen = true;
    }}>
      <Gear />
    </div>
  </div>
</div>

<style lang="sass">
  .titlebar
    background-color: var(--color-background-secondary)
    height: 50px
    width: 100%
    position: fixed
    top: 0
    left: 0
    z-index: 1000

    display: flex
    align-items: center
    justify-content: flex-start
    color: var(--color-text)
    letter-spacing: 1px
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2)

  .tabs
    display: flex
    align-items: center
    justify-content: center
    height: 100%
    margin-left: 20px
    margin-right: auto
    overflow-x: auto

  .quick-actions
    display: flex
    align-items: center
    justify-content: center
    height: 100%

  .quick-actions > div
    aspect-ratio: 1/1
    height: 90%
    cursor: pointer

    margin: 0 2px

    border-radius: 20%

    display: flex
    align-items: center
    justify-content: center


  .quick-actions > div:hover
    cursor: pointer
    background: #77777777

  .v-sep
    width: 1px
    height: 100%
    background: var(--color-border)

</style>

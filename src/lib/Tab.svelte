<script lang="ts">
  import { FileText } from "phosphor-svelte";
  import { currentTab, settings } from "../stores.js";

  export let tab: string;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class={"tab " + (tab === $currentTab ? "active" : "")} on:click={() => {
  $currentTab = tab;
}}>
  <FileText />
  <span>{tab.split(/[/\\]/g)[tab.split(/[/\\]/g).length - 1]}</span>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <span
    class="close-icon"
    on:click={(e) => {
      e.stopPropagation();
      $settings.opened_files = $settings.opened_files.filter((t) => t !== tab);
      if ($currentTab === tab) {
        if ($settings.opened_files.length > 0){
          $currentTab = $settings.opened_files[0];
        } else {
          $currentTab = "";
        }
      }
    }}>&times;</span
  >
</div>

<style lang="sass">
  .tab
    display: flex
    align-items: center
    justify-content: center
    height: 90%
    padding: 0 20px
    cursor: pointer
    background: #77777709
    border-bottom: 2px solid transparent
    transition: background 0.2s ease-in-out, border-bottom-color 0.2s ease-in-out

    &:hover,
    &:active
      background: #77777720

    span
      margin-left: 10px
  
  .active
    border-bottom-color: #777777
</style>

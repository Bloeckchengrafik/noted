<script lang="ts">
    import type {CtxMenu} from "../stores";
    import FileCtxMenu from "$lib/ctxmenu/FileCtxMenu.svelte";
    import {currentCtxMenuSettings} from "../stores.js";
    import DirCtxMenu from "$lib/ctxmenu/DirCtxMenu.svelte";

    export let ctxMenu: CtxMenu = {
        visible: false,
        x: 0,
        y: 0,
        payload: {type: "null"},
    };

    const done = () => {
        currentCtxMenuSettings.set({
            visible: false,
            x: 0,
            y: 0,
            payload: {type: "null"},
        });
    };
</script>

{#if ctxMenu.visible}
    <div class="ctx-menu-background" on:click={() => done()}>
        <div class="ctx-menu" style="left: {ctxMenu.x}px; top: {ctxMenu.y}px;" on:click={(e) => e.stopPropagation()}>
            {#if ctxMenu.payload.type === "file"}
                <FileCtxMenu ctxMenu={ctxMenu.payload} {done}/>
            {:else if ctxMenu.payload.type === "dir"}
                <DirCtxMenu ctxMenu={ctxMenu.payload} {done}/>
            {:else}
                <div>Unknown context menu type</div>
            {/if}
        </div>
    </div>
{/if}

<style lang="sass">
  .ctx-menu
    position: absolute
    background: var(--color-background-secondary)
    border: 1px solid var(--color-border)
    border-radius: 5px
    box-shadow: 0 0 1px 2px var(--color-background-secondary)
    padding: 5px

    user-select: none

    z-index: 50

    > :global(*)
      margin: 5px
      user-select: none


    > :global(*:first-child)
      margin-top: 0

    > :global(*:last-child)
      margin-bottom: 0

    > :global(hr)
      margin: 5px 0
      border: 0
      border-top: 1px solid var(--color-border)

    > :global(div)
      display: flex
      align-items: center
      cursor: pointer
      padding: 4px
      border-radius: 5px

      &:hover
        background: var(--color-background-tertiary)

      > :global(svg)
        margin-right: 5px

  .ctx-menu-background
    position: fixed
    top: 0
    left: 0
    width: 100vw
    height: 100vh
    z-index: 49

</style>
<script>
    import {hasSettingsOpen} from "../stores";
    import SettingsGeneral from "$lib/settings/SettingsGeneral.svelte";
    import {onDestroy, onMount} from "svelte";

    let openId = 0

    const onKeydown = (e) => {
        if (e.key === "Escape") {
            hasSettingsOpen.set(false)
        }

        if (e.key === "ArrowUp") {
            openId = openId - 1
            if (openId < 0) {
                openId = 0
            }
        }

        if (e.key === "ArrowDown") {
            openId = openId + 1
            if (openId > 2) {
                openId = 2
            }
        }
    }

    onMount(() => {
        document.addEventListener("keydown", onKeydown)
    })

    onDestroy(() => {
        document.removeEventListener("keydown", onKeydown)
    })
</script>

<div class="overlay" on:click={() => $hasSettingsOpen = false}>
    <div class="settings" on:click={(e) => e.stopPropagation()}>
        <div class="settings-header">
            <h2>Settings</h2>
            <button on:click={() => $hasSettingsOpen = false}>&times;</button>
        </div>

        <div class="settings-body">
            <div class="left-bar">
                <ul>
                    <li class={openId === 0 ? "active" : ""} on:click={() => {openId = 0}}>General</li>
                    <li class={openId === 1 ? "active" : ""} on:click={() => {openId = 1}}>Appearance</li>
                    <li class={openId === 2 ? "active" : ""} on:click={() => {openId = 2}}>Privacy</li>
                </ul>
            </div>
            <div class="right-bar">
                {#if openId === 0}
                    <h1>General</h1>
                    <SettingsGeneral />
                {:else if openId === 1}
                    <h1>Appearance</h1>
                {:else if openId === 2}
                    <h1>Privacy</h1>
                {/if}
            </div>
        </div>
    </div>
</div>

<style lang="sass">
  .overlay
    position: absolute
    top: 0
    left: 0
    width: 100%
    height: 100%
    z-index: 9999
    background: rgba(0, 0, 0, 0.2)

  .settings
    position: absolute
    top: 50%
    left: 50%
    transform: translate(-50%, -50%)
    width: 90%
    height: 80%
    background: var(--color-background-secondary)
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.5)

  .settings-header
    padding-top: 10px
    width: 100%
    height: 50px

    display: flex
    align-items: center
    justify-content: space-between

    > *
      margin: 0 20px

    h2
      font-size: 1.5rem
      font-weight: 600
      color: var(--color-text-primary)

    button
      font-size: 1.5rem
      font-weight: 600
      color: var(--color-text-primary)
      background: transparent
      border: none
      cursor: pointer

  .settings-body
    width: 100%
    height: calc(100% - 50px)
    padding: 20px
    overflow-y: auto
    display: flex
    flex-direction: row

  .left-bar
    height: calc(100% - 80px)
    width: 300px
    background: var(--color-background-primary)
    border-right: 1px solid var(--color-background-quaternary)

    ul
      list-style: none
      padding: 0
      margin: 0

      li
        padding: 10px 20px 10px 40px
        font-size: 1.2rem
        color: var(--color-text-primary)
        cursor: pointer
        background: transparent
        border-right: 3px solid transparent
        transition: background 0.2s ease, border-right-color 0.2s ease

        &:hover,
        &.active
          background: var(--color-background-tertiary)
          border-right-color: var(--color-primary)

  .right-bar
    height: calc(100% - 80px)
    width: calc(100% - 380px)
    background: var(--color-background-primary)
    padding: 0 20px 20px 20px
</style>
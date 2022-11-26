<script>
    import {Shadow, Moon, Circle3, Clock, Pulse, Stretch, Circle, Circle2, SyncLoader, Firework} from "svelte-loading-spinners";

    let loader_id = Math.floor(Math.random() * 10);
</script>

<div class="overlay" on:click={() => { loader_id = Math.floor(Math.random() * 10) }}>
    <div class="overlay-content">
        <div class="logo"></div>

        <br />

        {#if loader_id === 0}
            <Shadow color="var(--color-text)" size="100" unit="px" />
        {:else if loader_id === 1}
            <Moon color="var(--color-text)" size="100" unit="px" />
        {:else if loader_id === 2}
            <Circle3 size="100" unit="px" />
        {:else if loader_id === 3}
            <Clock color="var(--color-text)" size="100" unit="px" />
        {:else if loader_id === 4}
            <Pulse color="var(--color-text)" size="100" unit="px" />
        {:else if loader_id === 5}
            <Stretch color="var(--color-text)" size="100" unit="px" />
        {:else if loader_id === 6}
            <Circle color="var(--color-text)" size="100" unit="px" />
        {:else if loader_id === 7}
            <Circle2 size="100" unit="px" />
        {:else if loader_id === 8}
            <SyncLoader color="var(--color-text)" size="100" unit="px" />
        {:else if loader_id === 9}
            <Firework color="var(--color-text)" size="100" unit="px" />
        {/if}
    </div>
</div>


<style lang="sass">
  .overlay
    position: absolute
    top: 0
    left: 0
    width: 100%
    height: 100%
    background: var(--color-background-tertiary)
    z-index: 9999
    display: flex
    align-items: center
    justify-content: center

  .overlay-content
    text-align: center
    color: var(--color-text-primary)
    font-size: 1.5rem
    font-weight: 600
    padding: 1rem
    max-width: 500px
    margin: 0 auto

  .logo
    width: 100px
    height: 100px
    background: url('/favicon.png') no-repeat center center
    background-size: contain
    margin: 2rem auto 0

  html[theme="light"] .logo
    background: url('/favicon.for-light.png') no-repeat center center
    background-size: contain
</style>
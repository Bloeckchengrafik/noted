<script lang="ts">
    export let state: boolean;

    export function toggle() {
        state = !state;
    }
</script>

<!-- Toggle Button like Material UI -->

<button on:click={toggle}>
    <span class={state?"on":"off"}></span>
</button>

<style lang="sass">
  button
    width: 40px
    height: 15px
    border: none
    border-radius: 10px
    padding: 5px
    background: var(--color-background-quaternary)
    cursor: pointer
    display: inline-block
    position: relative
    transition: all 0.05s ease-in-out

  span
    position: absolute
    top: -2.5px
    left: -2.5px
    width: 20px
    height: 20px
    border-radius: 50%
    transition: all 0.15s ease-in-out

  .on
    background: var(--color-quinary)
    transform: translateX(25px)

  .off
    background: var(--color-secondary)
</style>

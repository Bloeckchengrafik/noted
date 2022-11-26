<script>
    import {Folder} from "phosphor-svelte";
</script>

<div class="tab">
    <Folder />
    <span>Folder</span>
    <span class="close-icon">&times;</span>
</div>

<style lang="sass">
  .tab
    display: flex
    align-items: center
    justify-content: center
    height: 70%
    width: 100px
    margin: 0 10px
    padding: 0 10px
    border-radius: 10px
    cursor: pointer
    transition: background-color 0.2s ease-in-out
    background: linear-gradient(90deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0.05) 100%)

    &:hover
      background-color: rgba(255, 255, 255, 0.1)

    &:active
      background-color: rgba(255, 255, 255, 0.2)

    span
      margin-left: 10px
</style>
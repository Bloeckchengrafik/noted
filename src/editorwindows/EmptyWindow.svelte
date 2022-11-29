<script lang="ts">
</script>

<div class="container">
    <img alt="" src="favicon.png">
    <div>
        <h1 style="">Welcome to Noted!</h1>
        <p style="color: var(--color-text-secondary)">The Note-Taking App for the 21st
            Century</p>
        <ul>
            <li>Use the vault view on the left to choose a file</li>
            <li>Switch between multiple files using the tabs</li>
            <li>Use the settings view to configure Noted to your liking</li>
        </ul>
    </div>
</div>

<style lang="sass">
  .container
    width: 100%
    height: calc(100% - 4em)
    display: flex
    flex-direction: row
    justify-content: center
    align-items: center
    text-align: left
    gap: 4em
    flex-wrap: wrap

    > div
      padding: 1em
      border-left: 2px solid var(--color-border)

  img
    width: 10em
    height: 10em
    margin-bottom: 1em

  ul
    margin: 0
    padding: 0
    list-style: none
    display: flex
    flex-direction: column
    justify-content: center
    align-items: center
    text-align: left
    width: 100%
    max-width: 30em

    li
      margin: 0.5em 0
      padding: 0
      width: 100%
      max-width: 30em
      font-size: 1.2em
</style>

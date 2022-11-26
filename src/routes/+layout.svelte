<script>
    import "../app.css";
    import Titlebar from "$lib/Titlebar.svelte";
    import Sidenav from "$lib/Filenav.svelte";
</script>

<main class="container">
    <Titlebar/>
    <Sidenav>
        <slot/>
    </Sidenav>
</main>

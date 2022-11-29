<script>
    import ToggleSwitch from "$lib/input/ToggleSwitch.svelte";
    import {onDestroy, onMount} from "svelte";

    let currentActive = -1;

    let refAutoFocus = null;
    let ref2AutoFocus = null;
    let settingAutoFocus = true;
    let setting2AutoFocus = true;

    let allRefs = [];

    $: allRefs = [
        refAutoFocus,
        ref2AutoFocus
    ];

    const updateActive = () => {
        if (currentActive === -1) {
            console.log("No active element");
            return;
        }

        for (let ref of allRefs) {
            if (!ref) continue;
            ref.style.borderLeftColor = "transparent";
        }

        allRefs[currentActive].style.borderLeftColor = "var(--color-primary)";

        // Un-Focus all elements
        for (let ref of allRefs) {
            if (!ref) continue;
            ref.blur();

            // Also children
            for (let child of ref.children) {
                child.blur();

                // Also children of children
                for (let child2 of child.children) {
                    child2.blur();
                }
            }
        }
    }

    const keyListener = (event) => {
        if (event.key === "Tab") {
            event.preventDefault();

            currentActive += event.ctrlKey ? -1 : 1;
            if (currentActive > allRefs.length - 1) {
                currentActive = 0;
            }
            if (currentActive < 0) {
                currentActive = allRefs.length - 1;
            }

            updateActive()
        }

        if (event.key === "Enter") {
            event.preventDefault();
            switch (currentActive) {
                case 0:
                    settingAutoFocus = !settingAutoFocus;
                    break;
                case 1:
                    setting2AutoFocus = !setting2AutoFocus;
                    break;
            }
        }
    };

    onMount(() => {
        document.addEventListener("keydown", keyListener);
        return () => {
            document.removeEventListener("keydown", keyListener);
        };
    });

    onDestroy(() => {
        document.removeEventListener("keydown", keyListener);
    });
</script>
<hr/>

<div class="row" bind:this={refAutoFocus}>
    <div class="component">
        <h5>Auto-Focus the current buffer</h5>
        <span>Choose whether to auto-focus the current buffer when nothing other is selected.</span>
    </div>
    <div class="component">
        <ToggleSwitch bind:state={settingAutoFocus}/>
    </div>
</div>
<hr/>
<div class="row" bind:this={ref2AutoFocus}>
    <div class="component">
        <h5>Auto-Focus the current buffer</h5>
        <span>Choose whether to auto-focus the current buffer when nothing other is selected.</span>
    </div>
    <div class="component">
        <ToggleSwitch bind:state={setting2AutoFocus}/>
    </div>
</div>
<hr/>

<style lang="sass">
  .row
    display: flex
    flex-direction: row
    align-items: center
    justify-content: space-between
    margin-bottom: 10px
    padding-left: 10px
    border-left: 2px solid transparent
    transition: border-left-color 0.2s ease-in-out

    .component
      display: flex
      flex-direction: column
      align-items: flex-start
      justify-content: center

      h5
        margin: 0
        font-size: 18px
        font-weight: 500

      span
        font-size: 12px
        color: var(--color-text-secondary)

  hr
    margin: 0 0 10px
    border: 0
    height: 0
    border-top: 1px solid var(--color-background-quaternary)
</style>
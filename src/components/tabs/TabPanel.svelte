<script lang="ts">
    import { getContext } from "svelte";
    import { TABS } from "./Tabs.svelte";

    const panel: Object = {};
    const { registerPanel, selectedPanel } = getContext(TABS);

    registerPanel(panel);
</script>

<div class="panel" class:hidden={$selectedPanel !== panel}>
    <slot></slot>
</div>
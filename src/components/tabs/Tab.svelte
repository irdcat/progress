<script lang="ts">
    import { getContext } from "svelte";
    import { TABS } from "./Tabs.svelte";

    const tab = {};
    const { registerTab, selectTab, selectedTab } = getContext(TABS);

    registerTab(tab);
</script>

<!-- svelte-ignore a11y-invalid-attribute -->
<a class="tab tab-bordered" class:tab-active={$selectedTab === tab} href="javascript:void(0)"
    on:click={() => selectTab(tab)}>
    <slot></slot>
</a>

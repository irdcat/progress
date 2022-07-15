<script lang="ts" context="module">
    export const TABS: Object = {};
</script>

<script lang="ts">
    import { setContext, onDestroy } from "svelte";
    import { writable, type Writable } from "svelte/store";

    const tabs: Object[] = [];
    const panels: Object[] = [];
    const selectedTab: Writable<Object> = writable(null);
    const selectedPanel: Writable<Object> = writable(null);

    setContext(TABS, {
        registerTab: tab => {
            tabs.push(tab);
            selectedTab.update(current => current || tab);

            onDestroy(() => {
                const i = tabs.indexOf(tab);
                tabs.splice(i, 1);
                selectedTab.update(current => {
                    return current === tab ? (tabs[i] || tabs[tabs.length - 1]) : current;
                });
            });
        },

        registerPanel: panel => {
            panels.push(panel);
            selectedPanel.update(current => current || panel);

            onDestroy(() => {
                const i = panels.indexOf(panel);
                panels.splice(i, 1);
                selectedPanel.update(current => {
                    return current === panel ? (panels[i] || panels[panels.length - 1]) : current;
                });
            });
        },

        selectTab: tab => {
            const i = tabs.indexOf(tab);
            selectedTab.set(tab);
            selectedPanel.set(panels[i]);
        },

        selectedTab,
        selectedPanel
    });
</script>

<div class="w-full">
    <slot></slot>
</div>

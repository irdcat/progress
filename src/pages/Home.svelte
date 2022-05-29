<script lang="ts">
    import { push } from "svelte-spa-router";
    import Button, { Label } from "@smui/button";
</script>

<Button variant="unelevated" on:click={() => push("/exercises")}>
    <Label>Exercise List</Label>
</Button>
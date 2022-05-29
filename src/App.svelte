<script lang="ts">
    import Button, { Label } from "@smui/button";
</script>

<main>
    <Button variant="unelevated" on:click={() => (alert("Clicked!"))}>
        <Label>Click me!</Label>
    </Button>
</main>
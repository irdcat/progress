<script lang="ts">
    import { push } from "svelte-spa-router";
</script>

<button class="btn" on:click={() => push("/exercises")}>Exercises</button>
<script lang="ts">
    import { push } from "svelte-spa-router";
</script>

<div class="hero min-h-full bg-base-300">
    <div class="hero-content text-center">
        <div class="max-w-md">
            <h1 class="text-5xl font-bold pb-8">Progress</h1>
            <button class="btn" on:click={() => push("/exercises")}>Exercises</button>
            <button class="btn" on:click={() => push("/trainings")}>Trainings</button>
        </div>
    </div>
</div>


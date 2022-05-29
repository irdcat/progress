<script lang="ts">
    import List, { Item, Text, Meta, Label as ItemLabel } from "@smui/list";
    import Button, { Label as ButtonLabel } from "@smui/button";
    import { push } from "svelte-spa-router";

    function goToExerciseEdit(exerciseId) {
        push(`/exercises/${exerciseId}/edit`);
    }

    function goToExerciseDetails(exerciseId) {
        push(`/exercises/${exerciseId}/details`)
    }

    // TODO: Fetch exercise list from backend
    let exercises = [
        { id: 1, name: "Deadlift" },
        { id: 2, name: "Barbell Row" },
        { id: 3, name: "Bench Press" },
        { id: 4, name: "Overhead Press" },
        { id: 5, name: "Squat" },
        { id: 6, name: "Romanian Deadlift"}
    ];
</script>

<style>
    .list-container {
        max-width: 600px;
    }
</style>

<div class="list-container">
    <List nonInteractive>
        {#each exercises as {id, name}, index (id)}
            <Item>
                <ItemLabel>{name}</ItemLabel>
                <Meta>
                    <Button variant="unelevated" on:click={() => goToExerciseEdit(id)}>
                        <ButtonLabel>Edit</ButtonLabel>
                    </Button>
                    <Button variant="unelevated" on:click={() => goToExerciseDetails(id)}>
                        <ButtonLabel>Details</ButtonLabel>
                    </Button>
                </Meta>
            </Item>
        {/each}
    </List>
</div>
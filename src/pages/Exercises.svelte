<script lang="ts">
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

<div class="m-auto max-w-5xl">
    <table class="table table-compact w-full">
        <thead>
            <tr>
                <th colspan="2">Exercises</th>
            </tr>
        </thead>
        <tbody>
            {#each exercises as {id, name}, index (id)}
                <tr>
                    <td>{name}</td>
                    <td class="w-14">
                        <button class="btn btn-primary btn-xs" on:click={() => goToExerciseEdit(id)}>Edit</button>
                        <button class="btn btn-primary btn-xs" on:click={() => goToExerciseDetails(id)}>Details</button>
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>
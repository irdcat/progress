<script lang="ts">
    import { querystring } from "svelte-spa-router";
    import { parse } from "qs";
    import type { Exercise, Training } from "../util/types";
    import { onMount } from "svelte";
    import ExerciseService from "../util/ExerciseService";
    import TrainingService from "../util/TrainingService";

    let params = parse($querystring);

    let exerciseService = new ExerciseService();
    let trainingService = new TrainingService();

    let exercises: Exercise[];
    let training: Training;

    onMount(async () => {
        exercises = await exerciseService.getExercises();
        training = await trainingService.getTraining(params.id);
    });
</script>

{#if training != undefined}
    <div class="w-full flex flex-col">
        <div class="flex grow p-2 bg-base-300">
            <p class="grow font-semibold text-xl leading-8 uppercase">
                Training {training.id}
            </p>
            <p class="grow-0 w-32 font-semibold text-xl leading-8">
                {training.date}
            </p>
        </div>
        <div class="flex grow p-2 bg-base-200">
            <p class="grow font-semibold text-lg leading-8 uppercase">
                Exercises
            </p>
        </div>
        {#each training.entries as entry}
            <div class="flex grow gap-x-4 p-2">
                <p class="grow-0 w-48 text-lg font-semibold p-2">
                    {exercises.filter((exercise) => exercise.id == entry.exercise_id)
                            .map((exercise) => exercise.name)}
                </p>
                <div class="flex grow-0 gap-x-3 p-2">
                    {#each entry.sets as set}
                        <p class="grow-0 font-medium">
                            {set.repetitions} x {set.weight}kg
                        </p>
                    {/each}
                </div>
            </div>
            <div class="flex grow gap-x-4 p-2">
                <div class="grow-0 w-48 p-2"></div>
                <p class="grow-0 font-medium p-2">
                    Volume {entry.sets.map((set) => set.repetitions * set.weight)
                        .reduce((total, setVolume) => total + setVolume)}kg
                </p>
            </div>
            <div class="flex grow-0 gap-x-4 p-2">
                <div class="grow-0 w-48 p-2"></div>    
                <p class="grow-0 font-medium p-2">
                    Average intensity {entry.sets.map((set) => set.weight)
                        .reduce((total, weight) => total + weight) / entry.sets.length}kg
                </p>
            </div>
        {/each}
    </div>
{/if}
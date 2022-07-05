<script lang="ts">
    import { querystring } from "svelte-spa-router";
    import { parse } from "qs";
    import type { Exercise, Training, TrainingEntry, TrainingSet } from "../util/types";
    import { onMount } from "svelte";
    import ExerciseFacade from "../util/ExerciseFacade";

    export let params = parse($querystring);

    const WEEKDAYS = ["Sunday","Monday","Tuesday","Wednesday","Thursday","Friday","Saturday"];

    let exerciseFacade = new ExerciseFacade();

    let exercises: Exercise[];
    let training: Training;

    function createTraining(): Training {
        let t: Training = {
            id: params.id,
            date: new Date(2022, 4, 22),
            exerciseEntries: []
        };

        for(let counter=0; counter < exercises.length; counter++) {
            let entry: TrainingEntry = {
                id: counter.toString(),
                exerciseId: exercises[counter].id,
                sets: []
            };
            for(let setCounter=0; setCounter < 3; setCounter++) {
                let set: TrainingSet = {
                    id: setCounter.toString(),
                    repetitions: 10,
                    weight: 60 + setCounter * 5
                };
                entry.sets.push(set);
            }
            t.exerciseEntries.push(entry);
        }

        return t;
    }

    onMount(async () => {
        let exerciseFacade = new ExerciseFacade();
        exercises = await exerciseFacade.getExercises();
        training = createTraining();
    })
</script>

{#if training != undefined}
    <div class="w-full flex flex-col">
        <div class="flex grow p-2 bg-base-300">
            <p class="grow font-semibold text-xl leading-8 uppercase">
                Training {training.id}
            </p>
            <p class="grow-0 w-32 font-semibold text-xl leading-8">
                {training.date.toISOString().substring(0, 10)}
            </p>
            <p class="grow-0 w-32 font-semibold text-xl leading-8">
                {WEEKDAYS[training.date.getDay()]}
            </p>
        </div>
        <div class="flex grow p-2 bg-base-200">
            <p class="grow font-semibold text-lg leading-8 uppercase">
                Exercises
            </p>
        </div>
        {#each training.exerciseEntries as entry}
            <div class="flex grow gap-x-4 p-2">
                <p class="grow-0 w-48 text-lg font-semibold p-2">
                    {exercises.filter((exercise) => exercise.id == entry.exerciseId)
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
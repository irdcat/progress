<script lang="ts">
    import { querystring } from "svelte-spa-router";
    import { parse } from "qs";
    import type { Exercise, Training, TrainingSet } from "../util/types";
    import { onMount } from "svelte";
    import ExerciseService from "../util/ExerciseService";
    import TrainingService from "../util/TrainingService";

    let params = parse($querystring);

    let exerciseService = new ExerciseService();
    let trainingService = new TrainingService();

    let exercises: Exercise[];
    let training: Training;

    function setsToString(sets: TrainingSet[]): string {
        let result: string = "";
        for(const [index, set] of sets.entries()) {
            result += set.repetitions.toString() + "x" + set.weight.toString() + "kg";
            if(index != sets.length - 1) {
                result += ", ";
            }
        }
        return result;
    }

    onMount(async () => {
        exercises = await exerciseService.getExercises();
        training = await trainingService.getTraining(params.id);
    });
</script>

{#if training != undefined}
    <div class="w-full flex flex-col">
        <div class="flex grow p-2 bg-base-300">
            <p class="grow font-semibold text-xl leading-8 uppercase">
                Training {training.date}
            </p>
        </div>
        <div class="flex grow p-2 bg-base-200">
            <p class="grow font-semibold text-lg leading-8 uppercase">
                Exercises
            </p>
        </div>
        <table class="table w-full">
            <thead>
                <th></th>
                <th>Name</th>
                <th>Sets</th>
                <th>Volume</th>
                <th>Average volume</th>
                <th>Average intensity</th>
            </thead>
            <tbody>
                {#each training.entries as entry, entryIndex}
                    <tr>
                        <th>{entryIndex + 1}</th>
                        <td>{exercises.filter(exercise => exercise.id == entry.exercise_id).map(exercise => exercise.name)}</td>
                        <td>{setsToString(entry.sets)}</td>
                        <td>{entry.sets.map((set) => set.repetitions * set.weight).reduce((total, setVolume) => total + setVolume)}kg</td>
                        <td>{(entry.sets.map((set) => set.repetitions * set.weight).reduce((total, setVolume) => total + setVolume) / entry.sets.length).toFixed(2)}kg</td>
                        <td>{(entry.sets.map((set) => set.weight).reduce((total, weight) => total + weight) / entry.sets.length).toFixed(2)}kg</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}

<script lang="ts">
    import { querystring } from "svelte-spa-router";
    import { parse } from "qs";
    import type { Exercise, Training, TrainingEntry, TrainingSet } from "../util/types";
    import { onMount } from "svelte";
    import ExerciseService from "../util/ExerciseService";
    import TrainingService from "../util/TrainingService";

    let params = parse($querystring);

    let exerciseService = new ExerciseService();
    let trainingService = new TrainingService();

    let exercises: Exercise[];
    let training: Training;

    function exerciseNameFromEntry(entry: TrainingEntry): string {
        return exercises
            .filter((exercise) => exercise.id == entry.exercise_id)
            .map((exercise) => exercise.name)
            .at(0);
    }

    async function setsFromEntry(entry: TrainingEntry): Promise<string> {
        let result: string = "";
        let exercise: Exercise = await exerciseService.getExercise(entry.exercise_id);
        
        if(exercise.bodyweight && entry.sets.filter((set) => set.weight > 0).length == 0) {
            for(const [index, set] of entry.sets.entries()) {
                result += set.repetitions.toString() + (index != entry.sets.length - 1 ? ", " : "");
            }
            return result;
        }

        if(exercise.double_weight) {
            for(const [index, set] of entry.sets.entries()) {
                let halfWeight: number = set.weight / 2;
                result += set.repetitions.toString() + "x(2x" 
                        + halfWeight.toString() + (index != entry.sets.length - 1 ? "), " : ")");  
            }
            return result;
        }

        for(const [index, set] of entry.sets.entries()) {
            result += set.repetitions.toString() + "x" + set.weight.toString() + "kg";
            if(index != entry.sets.length - 1) {
                result += ", ";
            }
        }
        return result;
    }

    async function volumeFromEntry(entry: TrainingEntry): Promise<string> {
        let exercise: Exercise = await exerciseService.getExercise(entry.exercise_id);
        if(exercise.bodyweight && entry.sets.filter((set) => set.weight > 0).length == 0) {
            return "-";
        }
        let totalVolume: number = entry.sets
            .map((set) => set.repetitions * set.weight)
            .reduce((total, setVolume) => total + setVolume);
        return totalVolume.toString() + "kg";
    }

    async function averageVolumeFromEntry(entry: TrainingEntry): Promise<string> {
        let exercise: Exercise = await exerciseService.getExercise(entry.exercise_id);
        if(exercise.bodyweight && entry.sets.filter((set) => set.weight > 0).length == 0) {
            return "-";
        }
        let totalVolume: number = entry.sets
            .map((set) => set.repetitions * set.weight)
            .reduce((total, setVolume) => total + setVolume);
        let averageVolume: number = totalVolume / entry.sets.length;
        return averageVolume.toFixed(2) + "kg";
    }

    async function averageIntensityFromEntry(entry: TrainingEntry): Promise<string> {
        let exercise: Exercise = await exerciseService.getExercise(entry.exercise_id);
        if(exercise.bodyweight && entry.sets.filter((set) => set.weight > 0).length == 0) {
            return "-";
        }
        let averageIntensity: number = entry.sets
            .map((set) => set.weight)
            .reduce((total, setIntensity) => total + setIntensity) / entry.sets.length;
        return averageIntensity.toFixed(2) + "kg";
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
                        <td>{exerciseNameFromEntry(entry)}</td>
                        <td>
                            {#await setsFromEntry(entry) then sets}
                                {sets}
                            {/await}
                        </td>
                        <td>
                            {#await volumeFromEntry(entry) then volume}
                                {volume}
                            {/await}
                        </td>
                        <td>
                            {#await averageVolumeFromEntry(entry) then averageVolume}
                                {averageVolume}
                            {/await}
                        </td>
                        <td>
                            {#await averageIntensityFromEntry(entry) then averageIntensity}
                                {averageIntensity}
                            {/await}
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}

<script lang="ts">
    import { LinkedChart, LinkedLabel, LinkedValue } from "svelte-tiny-linked-charts";
    import { querystring } from "svelte-spa-router";
    import { parse } from "qs";
    import ExerciseDetailsService from "../util/ExerciseDetailsService";
    import { onMount } from "svelte";
    import type { Exercise, ExerciseDetails } from "../util/types";
    import ExerciseService from "../util/ExerciseService";

    let params = parse($querystring)
    let exerciseService = new ExerciseService();
    let exerciseDetailsService = new ExerciseDetailsService();

    let volumes: Object = {};
    let avgVolumes: Object = {};
    let intensities: Object = {};

    let exercise: Exercise;

    onMount(async () => {
        type SetDetails = {
            repetitions: number,
            weight: number
        };

        exercise = await exerciseService.getExercise(params.id);
        let exerciseDetails: ExerciseDetails[] = await exerciseDetailsService.getExerciseDetails(params.id);
        let exerciseDetailsMap: Map<string, SetDetails[]> = new Map();
        
        for(const data of exerciseDetails) {
            if(exerciseDetailsMap.has(data.date)) {
                exerciseDetailsMap.get(data.date).push({repetitions: data.repetitions, weight: data.weight});
            } else {
                exerciseDetailsMap.set(data.date, [{repetitions: data.repetitions, weight: data.weight}]);
            }
        }

        for(const entry of exerciseDetailsMap) {
            let date = entry[0];
            let setDetails = entry[1];
            
            let volumeValue = setDetails.map((set) => set.repetitions * set.weight)
                .reduce((totalVolume, setVolume) => totalVolume + setVolume);
            volumes[date] = volumeValue.toFixed(2);
            let avgVolumeValue = volumes[date] / setDetails.length;
            avgVolumes[date] = avgVolumeValue.toFixed(2);
            let intensityValue = setDetails.map((set) => set.weight)
                .reduce((totalIntensity, intensity) => totalIntensity + intensity) / setDetails.length;
            intensities[date] = intensityValue.toFixed(2);
        }
    });
</script>

<div class="w-full flex flex-col">
    <div class="flex grow p-2 bg-base-300">
        <p class="grow font-semibold text-xl leading-8 uppercase">
            {#if exercise != undefined}
                Exercise: {exercise.name}
            {:else}
                Exercise {params.id}
            {/if}
        </p>
    </div>
    <div class="flex grow p-2">
        <p class="grow font-semibold text-lg leading-8 uppercase">
            Description
        </p>
    </div>
    <div class="flex grow p-2">
        <p class="grow">
            {#if exercise != undefined}
                {exercise.description}
            {/if}
        </p>
    </div>
    <div class="flex grow p-2 font-semibold">
        <p class="grow">
            {#if exercise != undefined}
                Bodyweight: {exercise.bodyweight ? "Yes" : "No"}
            {/if}
        </p>
        <p class="grow">
            {#if exercise != undefined}
                Unilateral: {exercise.unilateral ? "Yes" : "No"}
            {/if}
        </p>
        <p class="grow">
            {#if exercise != undefined}
                {exercise.double_weight ? "Double weight" : "Single weight"}
            {/if}
        </p>
    </div>
    <div class="flex grow p-2 bg-base-100">
        <p class="grow font-semibold text-lg leading-8 uppercase">
            Volume
        </p>
    </div>
    <div class="flex grow p-2 items-end">
        <div class="grow">
            {#if Object.keys(volumes).length == 0}
                No data
            {:else}
                <LinkedChart data={volumes} width="700" grow="true" height="100" align="left" fill="#20ff20" linked="link" uid="volume"/>
            {/if}
        </div>
        <p class="grow-0 w-28 text-left font-medium">
            <LinkedLabel linked="link"/>
        </p>
        <p class="grow-0 w-28 text-left font-medium">
            <LinkedValue uid="volume"/>
        </p>
    </div>
    <div class="flex grow p-2 bg-base-100">
        <p class="grow font-semibold text-lg leading-8 uppercase">
            Average volume per set
        </p>
    </div>
    <div class="flex grow p-2 items-end">
        <div class="grow">
            {#if Object.keys(avgVolumes).length == 0}
                No data
            {:else}
                <LinkedChart data={avgVolumes} width="700" grow="true" height="100" align="left" fill="#50ff50" linked="link" uid="avgVolume"/>
            {/if}
        </div>
        <p class="grow-0 w-28 text-left font-medium">
            <LinkedLabel linked="link"/>
        </p>
        <p class="grow-0 w-28 text-left font-medium">
            <LinkedValue uid="avgVolume"/>
        </p>
    </div>
    <div class="flex grow p-2 bg-base-100">
        <p class="grow font-semibold text-lg leading-8 uppercase">
            Average intensity pet set
        </p>
    </div>
    <div class="flex grow p-2 items-end">
        <div class="grow">
            {#if Object.keys(intensities).length == 0}
                No data
            {:else}
                <LinkedChart data={intensities} width="700" grow="true" height="100" align="left" fill="#ff2020" linked="link" uid="intensity"/>
            {/if}
        </div>
        <p class="grow-0 w-28 text-left font-medium">
            <LinkedLabel linked="link"/>
        </p>
        <p class="grow-0 w-28 text-left font-medium">
            <LinkedValue uid="intensity"/>
        </p>
    </div>
</div>

<script lang="ts">
    import type { TrainingEntry, TrainingPayload, TrainingSet } from "../util/types";
    import ModalUtils from "../util/ModalUtils";
    import StringUtils from "../util/StringUtils";
    import ExerciseFacade from "../util/ExerciseFacade";
    import { onMount } from "svelte";

    let exerciseFacade: ExerciseFacade = new ExerciseFacade();

    export let mId: string;
    export let caption: string = "";
    export let onOk: (data: TrainingPayload) => void = (d) => {};
    export let onCancel: (data: TrainingPayload) => void = (d) => {};

    const FORM_SUFFIX = "-form";
    const FORM_NAME = mId + FORM_SUFFIX;

    const ID_SUFFIX = "-id";
    const ID_FIELD_NAME = mId + ID_SUFFIX;
    const DATE_SUFFIX = "-date";
    const DATE_FIELD_NAME = mId + DATE_SUFFIX;
    const DAY_SUFFIX = "-day";
    const DAY_FIELD_NAME = mId + DAY_SUFFIX;
    const MONTH_SUFFIX = "-month";
    const MONTH_FIELD_NAME = mId + MONTH_SUFFIX;
    const YEAR_SUFFIX = "-year";
    const YEAR_FIELD_NAME = mId + YEAR_SUFFIX;
    
    const ENTRY_ID_SUFFIX_FORMAT = "-entry{0}-id";
    const ENTRY_ID_FIELD_NAME_FORMAT = mId + ENTRY_ID_SUFFIX_FORMAT;
    const ENTRY_EXERCISE_ID_SUFFIX_FORMAT = "-entry{0}-exercise-id";
    const ENTRY_EXERCISE_ID_FIELD_NAME_FORMAT = mId + ENTRY_EXERCISE_ID_SUFFIX_FORMAT;

    const ENTRY_SET_ID_SUFFIX_FORMAT = "-entry{0}-set{1}-id";
    const ENTRY_SET_ID_FIELD_NAME_FORMAT = mId + ENTRY_SET_ID_SUFFIX_FORMAT;
    const ENTRY_SET_REPETITIONS_SUFFIX_FORMAT = "-entry{0}-set{1}-repetitions";
    const ENTRY_SET_REPETITIONS_NAME_FIELD_FORMAT = mId + ENTRY_SET_REPETITIONS_SUFFIX_FORMAT;
    const ENTRY_SET_WEIGHT_SUFFIX_FORMAT = "-entry{0}-set{1}-weight";
    const ENTRY_SET_WEIGHT_FIELD_NAME_FORMAT = mId + ENTRY_SET_WEIGHT_SUFFIX_FORMAT;

    let entrySetCounter: number[] = [];
    let exercises: {id: string, name: string}[] = [];

    onMount(async () => {
        for(let exercise of (await exerciseFacade.getExercises())) {
            exercises.push({id: exercise.id, name: exercise.name});
        }
    });

    function cleanup(): void {
        entrySetCounter = [];
    }

    function addNewEntry(): void {
        entrySetCounter = [...entrySetCounter, 0];
    }

    function addNewEntrySet(entry: number): void {
        entrySetCounter[entry] += 1;
        entrySetCounter = [...entrySetCounter];
    }

    function payloadFromForm(modalId: string): TrainingPayload {
        let dateFromForm = new Date();
        dateFromForm.setFullYear(
            ModalUtils.getFormData(modalId, YEAR_SUFFIX.substring(1)),
            ModalUtils.getFormData(modalId, MONTH_SUFFIX.substring(1)) - 1,
            ModalUtils.getFormData(modalId, DAY_SUFFIX.substring(1)));

        let payload: TrainingPayload = {
            date: dateFromForm,
            exerciseEntries: []
        };

        for(let entryIndex = 0; entryIndex < entrySetCounter.length; entryIndex++) {
            let entryPayload: TrainingEntry = {
                id: ModalUtils.getFormData(modalId, 
                    StringUtils.format(ENTRY_ID_FIELD_NAME_FORMAT.substring(1), entryIndex)),
                exerciseId: ModalUtils.getFormData(modalId, 
                    StringUtils.format(ENTRY_EXERCISE_ID_SUFFIX_FORMAT.substring(1), entryIndex)),
                sets: []
            };
            for(let entrySetIndex = 0; entrySetIndex < entrySetCounter[entryIndex]; entrySetIndex++) {
                let setPayload: TrainingSet = {
                    id: ModalUtils.getFormData(modalId,
                        StringUtils.format(ENTRY_SET_ID_FIELD_NAME_FORMAT.substring(1), entryIndex, entrySetIndex)),
                    repetitions: ModalUtils.getFormData(modalId, 
                        StringUtils.format(ENTRY_SET_REPETITIONS_SUFFIX_FORMAT.substring(1), entryIndex, entrySetIndex)),
                    weight: ModalUtils.getFormData(modalId, 
                        StringUtils.format(ENTRY_SET_WEIGHT_SUFFIX_FORMAT.substring(1), entryIndex, entrySetIndex))
                };
                entryPayload.sets.push(setPayload);
            }
            payload.exerciseEntries.push(entryPayload);
        }

        return payload;
    }

    function ok(modalId: string): void {
        let data: TrainingPayload = payloadFromForm(modalId);
        onOk(data);
        ModalUtils.closeModal(modalId);
        cleanup();
    }

    function cancel(modalId: string): void {
        let data: TrainingPayload = payloadFromForm(modalId);
        onCancel(data);
        ModalUtils.closeModal(modalId);
        cleanup();
    }
</script>

<div id={mId} class="modal">
    <div class="modal-box w-11/12 max-w-5xl">
        {#if caption.length != 0}
            <h1 class="font-semibold text-2xl uppercase">
                {caption}
            </h1>
        {/if}
        <form name={FORM_NAME} id={FORM_NAME}>
            <input type="hidden" name={ID_FIELD_NAME} id={ID_FIELD_NAME}/>
            <div class="form-control w-full max-w-xs">
                <label for={DATE_FIELD_NAME} class="label">
                    <span class="label-text">Date</span>
                </label>
                <label class="input-group">
                    <input type="number" placeholder="Day" id={DAY_FIELD_NAME} name={DAY_FIELD_NAME} 
                        class="input input-bordered input-primary w-32 spin-button-none"
                        min="1" max="31"/>
                    <input type="number" placeholder="Month" id={MONTH_FIELD_NAME} name={MONTH_FIELD_NAME} 
                        class="input input-bordered input-primary w-32 spin-button-none"
                        min="1" max="12"/>
                    <input type="number" placeholder="Year" id={YEAR_FIELD_NAME} name={YEAR_FIELD_NAME} 
                        class="input input-bordered input-primary w-32 spin-button-none"
                        min="2020"/>
                </label>
            </div>
            <div class="w-full flex flex-col">
                <div class="flex grow p-2 bg-base-300">
                    <p class="grow font-semibold text-xl uppercase">
                        Exercises
                    </p>
                    <div class="grow-0">
                        <button type="button" class="btn btn-secondary btn-xs" on:click={() => addNewEntry()}>Add</button>
                    </div>
                </div>
                {#if entrySetCounter.length == 0}
                    <div class="flex grow p-2">
                        <p class="grow text-md font-normal">
                            No exercises
                        </p>
                    </div>
                {:else}
                    {#each entrySetCounter as entrySetCount, entryIndex}
                        <div class="flex flex-col grow p-2">
                            <div class="flex grow p-2">
                                <input type="hidden" value=""
                                    id={StringUtils.format(ENTRY_ID_FIELD_NAME_FORMAT, entryIndex)}
                                    name={StringUtils.format(ENTRY_ID_FIELD_NAME_FORMAT, entryIndex)}/>
                                <div class="form-control w-full max-w-xs">
                                    <label class="label" for={StringUtils.format(ENTRY_EXERCISE_ID_FIELD_NAME_FORMAT, entryIndex)}>
                                        <span class="label-text">Exercise</span>
                                    </label>
                                    <select class="select select-primary w-full max-w-xs"
                                        id={StringUtils.format(ENTRY_EXERCISE_ID_FIELD_NAME_FORMAT, entryIndex)}
                                        name={StringUtils.format(ENTRY_EXERCISE_ID_FIELD_NAME_FORMAT, entryIndex)}>
                                        {#each exercises as exercise}
                                            <option value={exercise.id}>{exercise.name}</option>
                                        {/each}
                                    </select>
                                </div>
                            </div>
                            <div class="flex grow p-2 bg-base-200">
                                <p class="grow font-semibold text-lg uppercase">
                                    Sets
                                </p>
                                <div class="grow-0">
                                    <button type="button" class="btn btn-secondary btn-xs" on:click={() => addNewEntrySet(entryIndex)}>Add</button>
                                </div>
                            </div>
                            {#if entrySetCounter[entryIndex] == 0}
                                <div class="flex grow p-2">
                                    <p class="grow text-md font-normal">
                                        No sets
                                    </p>
                                </div>
                            {:else}    
                                {#each Array(entrySetCount) as _, entrySetIndex}
                                    <div class="flex gap-x-3 grow p-2 items-end">
                                        <input type="hidden" value=""
                                            id={StringUtils.format(ENTRY_SET_ID_FIELD_NAME_FORMAT, entryIndex, entrySetIndex)}
                                            name={StringUtils.format(ENTRY_SET_ID_FIELD_NAME_FORMAT, entryIndex, entrySetIndex)}/>
                                        <div class="grow-0">
                                            <div class="form-control w-full max-w-xs">
                                                <label class="label" for={StringUtils.format(ENTRY_SET_REPETITIONS_NAME_FIELD_FORMAT, entryIndex, entrySetIndex)}>
                                                    <span class="label-text">Repetitions</span>
                                                </label>
                                                <input type="number"
                                                    class="input input-bordered input-primary w-32 spin-button-none"
                                                    placeholder="Reps"
                                                    id={StringUtils.format(ENTRY_SET_REPETITIONS_NAME_FIELD_FORMAT, entryIndex, entrySetIndex)}
                                                    name={StringUtils.format(ENTRY_SET_REPETITIONS_NAME_FIELD_FORMAT, entryIndex, entrySetIndex)}/>
                                            </div>
                                        </div>
                                        <div class="grow-0">
                                            <p class="text-lg font-semibold uppercase">
                                                X
                                            </p>
                                        </div>
                                        <div class="grow-0">
                                            <div class="form-control w-full max-w-xs">
                                                <label class="label" for={StringUtils.format(ENTRY_SET_WEIGHT_FIELD_NAME_FORMAT, entryIndex, entrySetIndex)}>
                                                    <span class="label-text">Weight</span>
                                                </label>
                                                <input type="number"
                                                    class="input input-bordered input-primary w-32 spin-button-none"
                                                    placeholder="Weight"
                                                    id={StringUtils.format(ENTRY_SET_WEIGHT_FIELD_NAME_FORMAT, entryIndex, entrySetIndex)}
                                                    name={StringUtils.format(ENTRY_SET_WEIGHT_FIELD_NAME_FORMAT, entryIndex, entrySetIndex)}/>
                                            </div>
                                        </div>
                                    </div>
                                {/each}
                            {/if}
                        </div>
                    {/each}
                {/if}
            </div>
        </form>
        <div class="flex">
            <div class="grow"/>
            <div class="grow-0 modal-action">
                <button class="btn btn-success" on:click={() => ok(mId)}>Ok</button>
                <button class="btn btn-error" on:click={() => cancel(mId)}>Cancel</button>
            </div>
        </div>
    </div>
</div>
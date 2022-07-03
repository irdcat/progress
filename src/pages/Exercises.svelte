<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import ExerciseModal, { ExerciseModalUtils } from "../components/ExerciseModal.svelte";
    import ExerciseFacade from "../util/ExerciseFacade";
    import type { Exercise, ExercisePayload } from "../util/types";

    let exerciseFacade = new ExerciseFacade();
    let exercises: Exercise[] = [];

    const EXERCISE_ADD_MODAL_ID = "exercise-add-modal";
    const EXERCISE_EDIT_MODAL_ID = "exercise-edit-modal";

    onMount(async () => {
        exercises = await exerciseFacade.getExercises();
    });

    function goToExerciseDetails(exerciseId): void {
        push(`/exercises/details?id=${exerciseId}`)
    }

    function openExerciseAddModal(): void {
        ExerciseModalUtils.openModal(EXERCISE_ADD_MODAL_ID);
    }

    async function openExerciseEditModal(id: string): Promise<void> {
        let exercise: Exercise = await exerciseFacade.getExercise(id);
        ExerciseModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "id", exercise.id);
        ExerciseModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "name", exercise.name);
        ExerciseModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "description", exercise.description);
        ExerciseModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "bodyweight", exercise.bodyweight);
        ExerciseModalUtils.openModal(EXERCISE_EDIT_MODAL_ID);
    }

    async function onAddOk(data: ExercisePayload): Promise<void> {
        await exerciseFacade.addExercise(data);
        exercises = await exerciseFacade.getExercises();
    }

    async function onEditOk(data: ExercisePayload): Promise<void> {
        let exerciseId: string = ExerciseModalUtils.getFormData(EXERCISE_EDIT_MODAL_ID, "id");
        await exerciseFacade.updateExercise(exerciseId, data);
        exercises = await exerciseFacade.getExercises();
    }
</script>

<ExerciseModal mId={EXERCISE_ADD_MODAL_ID} onOk={onAddOk}/>
<ExerciseModal mId={EXERCISE_EDIT_MODAL_ID} onOk={onEditOk}/>

<div class="w-full flex flex-col">
    <div class="flex grow p-2 bg-base-300">
        <p class="grow font-semibold text-2xl leading-8 uppercase">
            Exercises
        </p>
        <div class="grow-0">
            <button class="btn btn-primary btn-sm" on:click={() => openExerciseAddModal()}>Add</button>
        </div>
    </div>
    {#each exercises as {id, name}, index (id)}
        <div class="flex grow p-2">
            <p class="grow text-md font-normal">
                {name}
            </p>
            <div class="grow-0">
                <button class="btn btn-secondary btn-sm" on:click={() => openExerciseEditModal(id)}>Edit</button>
                <button class="btn btn-secondary btn-sm" on:click={() => goToExerciseDetails(id)}>Details</button>
            </div>
        </div>
    {/each}
</div>
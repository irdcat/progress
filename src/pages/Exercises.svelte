<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import ExerciseModal from "../components/ExerciseModal.svelte";
    import ExerciseFacade from "../util/ExerciseFacade";
    import ModalUtils from "../util/ModalUtils";
    import type { Exercise, ExercisePayload } from "../util/types";

    let exerciseFacade: ExerciseFacade = new ExerciseFacade();
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
        ModalUtils.openModal(EXERCISE_ADD_MODAL_ID);
    }

    async function openExerciseEditModal(id: string): Promise<void> {
        let exercise: Exercise = await exerciseFacade.getExercise(id);
        ModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "id", exercise.id);
        ModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "name", exercise.name);
        ModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "description", exercise.description);
        ModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "bodyweight", exercise.bodyweight);
        ModalUtils.openModal(EXERCISE_EDIT_MODAL_ID);
    }

    async function onAddOk(data: ExercisePayload): Promise<void> {
        await exerciseFacade.addExercise(data);
        exercises = await exerciseFacade.getExercises();
    }

    async function onEditOk(data: ExercisePayload): Promise<void> {
        let exerciseId: string = ModalUtils.getFormData(EXERCISE_EDIT_MODAL_ID, "id");
        await exerciseFacade.updateExercise(exerciseId, data);
        exercises = await exerciseFacade.getExercises();
    }
</script>

<ExerciseModal mId={EXERCISE_ADD_MODAL_ID} caption="Add exercise" onOk={onAddOk}/>
<ExerciseModal mId={EXERCISE_EDIT_MODAL_ID} caption="Edit exercise" onOk={onEditOk}/>

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
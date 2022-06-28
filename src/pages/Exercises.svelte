<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import ExerciseModal, { getFormData, openModal, setFormData } from "../components/ExerciseModal.svelte";
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
        openModal(EXERCISE_ADD_MODAL_ID);
    }

    async function openExerciseEditModal(id: string): Promise<void> {
        let exercise: Exercise = await exerciseFacade.getExercise(id);
        setFormData(EXERCISE_EDIT_MODAL_ID, "id", exercise.id);
        setFormData(EXERCISE_EDIT_MODAL_ID, "name", exercise.name);
        setFormData(EXERCISE_EDIT_MODAL_ID, "description", exercise.description);
        setFormData(EXERCISE_EDIT_MODAL_ID, "bodyweight", exercise.bodyweight);
        openModal(EXERCISE_EDIT_MODAL_ID);
    }

    async function onAddOk(data: ExercisePayload): Promise<void> {
        await exerciseFacade.addExercise(data);
    }

    async function onEditOk(data: ExercisePayload): Promise<void> {
        let exerciseId: string = getFormData(EXERCISE_EDIT_MODAL_ID, "id");
        await exerciseFacade.updateExercise(exerciseId, data);
    }
</script>

<ExerciseModal mId={EXERCISE_ADD_MODAL_ID} onOk={onAddOk}/>
<ExerciseModal mId={EXERCISE_EDIT_MODAL_ID} onOk={onEditOk}/>

<table class="table table-compact w-full">
    <thead>
        <tr>
            <th>Exercises</th>
            <th class="w-14">
            </th>
            <th class="w-14">
                <button class="btn btn-primary btn-xs" on:click={() => openExerciseAddModal()}>Add</button>
            </th>
        </tr>
    </thead>
    <tbody>
        {#each exercises as {id, name}, index (id)}
            <tr>
                <td>{name}</td>
                <td colspan="2" class="w-14">
                    <button class="btn btn-secondary btn-xs" on:click={() => openExerciseEditModal(id)}>Edit</button>
                    <button class="btn btn-secondary btn-xs" on:click={() => goToExerciseDetails(id)}>Details</button>
                </td>
            </tr>
        {/each}
    </tbody>
</table>
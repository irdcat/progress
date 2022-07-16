<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import DeleteExerciseModal from "../components/DeleteExerciseModal.svelte";
    import ExerciseModal from "../components/ExerciseModal.svelte";
    import ExerciseService from "../util/ExerciseService";
    import ModalUtils from "../util/ModalUtils";
    import type { Exercise, ExercisePayload } from "../util/types";

    let exerciseService: ExerciseService = new ExerciseService();
    let exercises: Exercise[] = [];

    const EXERCISE_ADD_MODAL_ID = "exercise-add-modal";
    const EXERCISE_EDIT_MODAL_ID = "exercise-edit-modal";
    const EXERCISE_DELETE_MODAL_ID = "exercise-delete-modal";

    onMount(async () => {
        exercises = await exerciseService.getExercises();
    });

    function goToExerciseDetails(exerciseId): void {
        push(`/exercises/details?id=${exerciseId}`)
    }

    function openExerciseAddModal(): void {
        ModalUtils.openModal(EXERCISE_ADD_MODAL_ID);
    }

    async function openExerciseEditModal(id: string): Promise<void> {
        let exercise: Exercise = await exerciseService.getExercise(id);
        ModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "id", exercise.id);
        ModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "name", exercise.name);
        ModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "description", exercise.description);
        ModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "bodyweight", exercise.bodyweight);
        ModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "unilateral", exercise.unilateral);
        ModalUtils.setFormData(EXERCISE_EDIT_MODAL_ID, "double-weight", exercise.double_weight);
        ModalUtils.openModal(EXERCISE_EDIT_MODAL_ID);
    }

    function openExerciseDeleteModal(id: string): void {
        ModalUtils.setFormData(EXERCISE_DELETE_MODAL_ID, "exercise-id", id);
        ModalUtils.openModal(EXERCISE_DELETE_MODAL_ID);
    }

    async function onAddOk(data: ExercisePayload): Promise<void> {
        await exerciseService.addExercise(data);
        exercises = await exerciseService.getExercises();
    }

    async function onEditOk(data: ExercisePayload): Promise<void> {
        let exerciseId: string = ModalUtils.getFormData(EXERCISE_EDIT_MODAL_ID, "id");
        await exerciseService.updateExercise(exerciseId, data);
        exercises = await exerciseService.getExercises();
    }

    async function onDeleteOk(id: string): Promise<void> {
        await exerciseService.deleteExercise(id);
        exercises = await exerciseService.getExercises();
    }
</script>

<ExerciseModal mId={EXERCISE_ADD_MODAL_ID} caption="Add exercise" onOk={onAddOk}/>
<ExerciseModal mId={EXERCISE_EDIT_MODAL_ID} caption="Edit exercise" onOk={onEditOk}/>
<DeleteExerciseModal mId={EXERCISE_DELETE_MODAL_ID} onYes={onDeleteOk}/>

<div class="w-full flex flex-col">
    <div class="flex grow p-2 bg-base-300">
        <p class="grow font-semibold text-2xl leading-8 uppercase">
            Exercises
        </p>
        <div class="grow-0">
            <button class="btn btn-primary btn-sm" on:click={() => openExerciseAddModal()}>Add</button>
        </div>
    </div>
    <table class="table w-full">
        <thead>
            <th>Name</th>
            <th>Bodyweight</th>
            <th>Unilateral</th>
            <th>Double weight</th>
            <th><!--actions--></th>
        </thead>
        <tbody>
            {#each exercises as {id, name, unilateral, bodyweight, double_weight}, index (id)}
                <tr>
                    <td>{name}</td>
                    <td>{bodyweight ? "Yes" : "No"}</td>
                    <td>{unilateral ? "Yes" : "No"}</td>
                    <td>{double_weight ? "Yes" : "No"}</td>
                    <td class="w-20">
                        <div class="dropdown dropdown-end">
                            <!-- svelte-ignore a11y-label-has-associated-control -->
                            <label tabindex="0" class="btn btn-ghost btn-xs">
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-three-dots" viewBox="0 0 16 16">
                                    <path d="M3 9.5a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm5 0a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm5 0a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3z"/>
                                </svg>
                            </label>
                            <ul tabindex="0" class="dropdown-content p-2 menu shadow bg-base-100 rounded-box w-fit">
                                <li on:click={() => openExerciseEditModal(id)}>
                                    <!-- svelte-ignore a11y-missing-attribute -->
                                    <a>Edit</a>
                                </li>
                                <li on:click={() => goToExerciseDetails(id)}>
                                    <!-- svelte-ignore a11y-missing-attribute -->
                                    <a>Details</a>
                                </li>
                                <li on:click={() => openExerciseDeleteModal(id)}>
                                    <!-- svelte-ignore a11y-missing-attribute -->
                                    <a class="hover:bg-red-700">Delete</a>
                                </li>
                            </ul>
                        </div>
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>

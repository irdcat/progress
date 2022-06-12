<script lang="ts">
    import { push } from "svelte-spa-router";

    function goToExerciseDetails(exerciseId) {
        push(`/exercises/details?id=${exerciseId}`)
    }

    // TODO: Fetch exercise list from backend
    let exercises = [
        { id: 1, name: "Deadlift" },
        { id: 2, name: "Barbell Row" },
        { id: 3, name: "Bench Press" },
        { id: 4, name: "Overhead Press" },
        { id: 5, name: "Squat" },
        { id: 6, name: "Romanian Deadlift"}
    ];

    function resolveHighestExerciseId() :number {
        let idsArray = exercises.map((exercise) => exercise.id);
        return Math.max.apply(null, idsArray);
    }

    function openExerciseAddModal() {
        let modalElement = document.querySelector("#exercise-add-modal");
        modalElement.classList.add("modal-open");
    }

    function openExerciseEditModal(exerciseId: number) {
        let nameElement: HTMLInputElement = document.querySelector("input#edit-name");
        nameElement.value = exercises.find((exercise) => exercise.id == exerciseId).name;

        let idElement: HTMLInputElement = document.querySelector("input#edit-id");
        idElement.value = exerciseId.toString();

        let modalElement = document.querySelector("#exercise-edit-modal");
        modalElement.classList.add("modal-open");
    }

    function closeExerciseAddModal() {
        let modalElement = document.querySelector("#exercise-add-modal");
        modalElement.classList.remove("modal-open");
    }

    function closeExerciseEditModal() {
        let modalElement = document.querySelector("#exercise-edit-modal");
        modalElement.classList.remove("modal-open");
    }

    function handleExerciseAddModalOk() {
        let nameElement: HTMLInputElement = document.querySelector("input#name");
        if(nameElement.value == '' || nameElement.value == undefined) {
            let nameErrorElement: HTMLSpanElement = document.querySelector("span#name-error");
            nameErrorElement.innerText = "Name cannot be empty";
            return;
        }

        let exerciseName: string = nameElement.value;
        let highestId: number = resolveHighestExerciseId() + 1;
        exercises = [...exercises, {id: highestId, name: exerciseName}]
        closeExerciseAddModal();
    }

    function handleExerciseEditModalOk() {
        let nameElement: HTMLInputElement = document.querySelector("input#edit-name");
        if(nameElement.value == '' || nameElement.value == undefined) {
            let nameErrorElement: HTMLSpanElement = document.querySelector("span#name-error");
            nameErrorElement.innerText = "Name cannot be empty";
            return;
        }

        let idElement: HTMLInputElement = document.querySelector("input#edit-id");

        let exerciseName = nameElement.value;
        let exerciseId = parseInt(idElement.value);
        exercises = [...exercises.filter((exercise) => exercise.id != exerciseId), {id: exerciseId, name: exerciseName}]
            .sort((exerciseA, exerciseB) => exerciseA.id - exerciseB.id);
        closeExerciseEditModal();
    }
</script>

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

<div id="exercise-add-modal" class="modal">
    <div class="modal-box w-11/12 max-w-5xl">
        <div class="form-control w-full max-w-xs">
            <label for="name" class="label">
                <span class="label-text">Exercise name</span>
            </label>
            <input type="text" placeholder="Name" id="name" class="input input-bordered input-primary w-full max-w-xs"/>
            <label for="name" class="label">
                <span id="name-error" class="label-text-alt text-red-600"></span>
            </label>
        </div>
        <div class="flex">
            <div class="grow"></div>
            <div class="flex-none modal-action">
                <button class="btn btn-success" on:click={() => handleExerciseAddModalOk()}>Ok</button>
                <button class="btn btn-error" on:click={() => closeExerciseAddModal()}>Cancel</button>
            </div>
        </div>
    </div>
</div>

<div id="exercise-edit-modal" class="modal">
    <div class="modal-box w-11/12 max-w-5xl">
        <input type="hidden" id="edit-id"/>
        <div class="form-control w-full max-w-xs">
            <label for="edit-name" class="label">
                <span class="label-text">Exercise name</span>
            </label>
            <input type="text" placeholder="Name" id="edit-name" class="input input-bordered input-primary w-full max-w-xs"/>
            <label for="edit-name" class="label">
                <span id="edit-name-error" class="label-text-alt text-red-600"></span>
            </label>
        </div>
        <div class="flex">
            <div class="grow"></div>
            <div class="flex-none modal-action">
                <button class="btn btn-success" on:click={() => handleExerciseEditModalOk()}>Ok</button>
                <button class="btn btn-error" on:click={() => closeExerciseEditModal()}>Cancel</button>
            </div>
        </div>
    </div>
</div>
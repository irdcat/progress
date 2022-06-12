<script lang="ts">
    import { push } from "svelte-spa-router";

    function goToExerciseEdit(exerciseId) {
        push(`/exercises/${exerciseId}/edit`);
    }

    function goToExerciseDetails(exerciseId) {
        push(`/exercises/${exerciseId}/details`)
    }

    function openModal() {
        let modalElement = document.querySelector("#app-modal");
        modalElement.classList.add("modal-open");
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

    function closeModal() {
        let modalElement = document.querySelector("#app-modal");
        modalElement.classList.remove("modal-open");
    }

    function handleModalOk() {
        let nameElement: HTMLInputElement = document.querySelector("input#name");
        if(nameElement.value == '' || nameElement.value == undefined) {
            let nameErrorElement: HTMLSpanElement = document.querySelector("span#name-error");
            nameErrorElement.innerText = "Name cannot be empty";
            return;
        }

        let exerciseName: string = nameElement.value;
        let highestId: number = resolveHighestExerciseId() + 1;
        exercises = [...exercises, {id: highestId, name: exerciseName}]
        closeModal();
    }
</script>

<table class="table table-compact w-full">
    <thead>
        <tr>
            <th>Exercises</th>
            <th class="w-14">
            </th>
            <th class="w-14">
                <button class="btn btn-primary btn-sm" on:click={() => openModal()}>Add</button>
            </th>
        </tr>
    </thead>
    <tbody>
        {#each exercises as {id, name}, index (id)}
            <tr>
                <td>{name}</td>
                <td colspan="2" class="w-14">
                    <button class="btn btn-primary btn-xs" on:click={() => goToExerciseEdit(id)}>Edit</button>
                    <button class="btn btn-primary btn-xs" on:click={() => goToExerciseDetails(id)}>Details</button>
                </td>
            </tr>
        {/each}
    </tbody>
</table>

<div id="app-modal" class="modal">
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
        <div class="form control w-full max-w-xs">
            <label for="description" class="label">
                <span class="label-text">Exercise description</span>
            </label>
            <textarea id="description" class="textarea textarea-primary w-full max-w-xs" placeholder="Description"></textarea>
        </div>
        <div class="flex">
            <div class="grow"></div>
            <div class="flex-none modal-action">
                <label for="app-modal" class="btn btn-success" on:click={() => handleModalOk()}>Ok</label>
                <label for="app-modal" class="btn btn-error" on:click={() => closeModal()}>Cancel</label>
            </div>
        </div>
    </div>
</div>
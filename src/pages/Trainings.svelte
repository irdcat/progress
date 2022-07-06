<script lang="ts">
    import { push } from "svelte-spa-router";
    import TrainingModal from "../components/TrainingModal.svelte";
    import ModalUtils from "../util/ModalUtils";
import type { TrainingPayload } from "../util/types";

    const TRAINING_ADD_MODAL = "training-add-modal";
    const TRAINING_EDIT_MODAL = "training-edit-modal";

    function formatDate(date: Date): string {
        return date.toISOString().substring(0, 10);
    }

    //TODO: Fetch trainings from backend
    let trainings = [
        {id: "1", date: formatDate(new Date(2022, 5, 14))},
        {id: "2", date: formatDate(new Date(2022, 5, 16))},
        {id: "3", date: formatDate(new Date(2022, 5, 18))},
        {id: "4", date: formatDate(new Date(2022, 5, 21))},
        {id: "5", date: formatDate(new Date(2022, 5, 23))}
    ];

    function openTrainingAddModal(): void {
        ModalUtils.openModal(TRAINING_ADD_MODAL);
    }

    function openTrainingEditModal(id: string): void {
        // TODO: Populate modal form data based on output from backend
        ModalUtils.openModal(TRAINING_EDIT_MODAL);
    }

    async function onAddOk(data: TrainingPayload): Promise<void> {
        // TODO: Send form data to backend
    }

    async function onEditOk(data: TrainingPayload): Promise<void> {
        // TODO: Send form data to backend
    }

    function goToTrainingDetails(trainingId: string): void {
        push(`/trainings/details?id=${trainingId}`);
    }
</script>

<TrainingModal mId={TRAINING_ADD_MODAL} onOk={onAddOk} caption="Add training"/>
<TrainingModal mId={TRAINING_EDIT_MODAL} onOk={onEditOk} caption="Edit training"/>

<div class="w-full flex flex-col">
    <div class="flex grow p-2 bg-base-300">
        <p class="grow font-semibold text-2xl leading-8 uppercase">
            Trainings
        </p>
        <div class="grow-0">
            <button class="btn btn-primary btn-sm" on:click={() => openTrainingAddModal()}>Add</button>
        </div>
    </div>
    {#each trainings as {id, date}, index (id)}
        <div class="flex grow p-2">
            <p class="grow text-md font-normal">
                {date}
            </p>
            <div class="grow-0">
                <button class="btn btn-secondary btn-sm" on:click={() => openTrainingEditModal(id)}>Edit</button>
                <button class="btn btn-secondary btn-sm" on:click={() => goToTrainingDetails(id)}>Details</button>
            </div>
        </div>
    {/each}
</div>
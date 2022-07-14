<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import TrainingModal from "../components/TrainingModal.svelte";
    import TrainingService from "../util/TrainingService";
    import ModalUtils from "../util/ModalUtils";
    import type { Training, TrainingPayload } from "../util/types";

    let trainingService: TrainingService = new TrainingService();
    let trainings: Training[] = [];

    const TRAINING_ADD_MODAL_ID = "training-add-modal";
    const TRAINING_EDIT_MODAL_ID = "training-edit-modal";

    onMount(async () => {
        trainings = await trainingService.getTrainings();
    });

    function openTrainingAddModal(): void {
        ModalUtils.openModal(TRAINING_ADD_MODAL_ID);
    }

    async function openTrainingEditModal(id: string): Promise<void> {
        let training: Training = await trainingService.getTraining(id);
        // TODO: Populate form data
        ModalUtils.openModal(TRAINING_EDIT_MODAL_ID);
    }

    async function onAddOk(data: TrainingPayload): Promise<void> {
        await trainingService.addTraining(data);
        trainings = await trainingService.getTrainings();
    }

    async function onEditOk(data: TrainingPayload): Promise<void> {
        let trainingId: string = ModalUtils.getFormData(TRAINING_EDIT_MODAL_ID, "id");
        await trainingService.updateTraining(trainingId, data);
        trainings = await trainingService.getTrainings();
    }

    function goToTrainingDetails(trainingId: string): void {
        push(`/trainings/details?id=${trainingId}`);
    }
</script>

<TrainingModal mId={TRAINING_ADD_MODAL_ID} onOk={onAddOk} caption="Add training"/>
<TrainingModal mId={TRAINING_EDIT_MODAL_ID} onOk={onEditOk} caption="Edit training"/>

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
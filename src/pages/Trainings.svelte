<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import DeleteTrainingModal from "../components/DeleteTrainingModal.svelte";
    import TrainingModal, { TrainingModalUtils } from "../components/TrainingModal.svelte";
    import ModalUtils from "../util/ModalUtils";
    import sleep from "../util/sleep";
    import StringUtils from "../util/StringUtils";
    import TrainingService from "../util/TrainingService";
    import type { Training, TrainingPayload } from "../util/types";

    let trainingService: TrainingService = new TrainingService();
    let trainings: Training[] = [];

    const TRAINING_ADD_MODAL_ID = "training-add-modal";
    const TRAINING_EDIT_MODAL_ID = "training-edit-modal";
    const TRAINING_DELETE_MODAL_ID = "training-delete-modal";

    onMount(async () => {
        trainings = await trainingService.getTrainings();
    });

    function openTrainingAddModal(): void {
        TrainingModalUtils.openModal(TRAINING_ADD_MODAL_ID);
    }

    async function openTrainingEditModal(id: string): Promise<void> {
        let training: Training = await trainingService.getTraining(id);
        TrainingModalUtils.setEntryCount(TRAINING_EDIT_MODAL_ID, training.entries.length);
        for(const [index, entry] of training.entries.entries()) {
            TrainingModalUtils.setEntrySetCount(TRAINING_EDIT_MODAL_ID, index, entry.sets.length);
        }
        sleep(50).then(() => {
            let dateParts = training.date.split('-');
            TrainingModalUtils.setFormData(TRAINING_EDIT_MODAL_ID, "id", training.id);
            TrainingModalUtils.setFormData(TRAINING_EDIT_MODAL_ID, "year", dateParts[0]);
            TrainingModalUtils.setFormData(TRAINING_EDIT_MODAL_ID, "month", parseInt(dateParts[1]));
            TrainingModalUtils.setFormData(TRAINING_EDIT_MODAL_ID, "day", dateParts[2]);
            for(const [entryIndex, entry] of training.entries.entries()) {
                TrainingModalUtils.setFormData(TRAINING_EDIT_MODAL_ID, 
                    StringUtils.format("entry{0}-id", entryIndex), entry.id);
                TrainingModalUtils.setFormData(TRAINING_EDIT_MODAL_ID, 
                    StringUtils.format("entry{0}-exercise-id", entryIndex), entry.exercise_id);
                for(const [setIndex, set] of training.entries[entryIndex].sets.entries()) {
                    TrainingModalUtils.setFormData(TRAINING_EDIT_MODAL_ID, 
                        StringUtils.format("entry{0}-set{1}-id", entryIndex, setIndex), set.id);
                    TrainingModalUtils.setFormData(TRAINING_EDIT_MODAL_ID, 
                        StringUtils.format("entry{0}-set{1}-repetitions", entryIndex, setIndex), set.repetitions);
                    TrainingModalUtils.setFormData(TRAINING_EDIT_MODAL_ID, 
                        StringUtils.format("entry{0}-set{1}-weight", entryIndex, setIndex), set.weight);
                }
            }
        });
        TrainingModalUtils.openModal(TRAINING_EDIT_MODAL_ID);
    }

    function openTrainingDeleteModal(id: string): void {
        ModalUtils.setFormData(TRAINING_DELETE_MODAL_ID, "training-id", id);
        ModalUtils.openModal(TRAINING_DELETE_MODAL_ID);
    }

    async function onAddOk(data: TrainingPayload): Promise<void> {
        await trainingService.addTraining(data);
        trainings = await trainingService.getTrainings();
    }

    async function onEditOk(data: TrainingPayload): Promise<void> {
        let trainingId: string = TrainingModalUtils.getFormData(TRAINING_EDIT_MODAL_ID, "id");
        await trainingService.updateTraining(trainingId, data);
        trainings = await trainingService.getTrainings();
    }

    async function onDeleteOk(id: string): Promise<void> {
        await trainingService.deleteTraining(id);
        trainings = await trainingService.getTrainings();
    }

    function goToTrainingDetails(trainingId: string): void {
        push(`/trainings/details?id=${trainingId}`);
    }
</script>

<TrainingModal mId={TRAINING_ADD_MODAL_ID} onOk={onAddOk} caption="Add training"/>
<TrainingModal mId={TRAINING_EDIT_MODAL_ID} onOk={onEditOk} caption="Edit training"/>
<DeleteTrainingModal mId={TRAINING_DELETE_MODAL_ID} onYes={onDeleteOk}/>

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
                <button class="btn btn-error btn-sm" on:click={() => openTrainingDeleteModal(id)}>Delete</button>
            </div>
        </div>
    {/each}
</div>

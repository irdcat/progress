<script lang="ts">
    import ModalUtils from "../util/ModalUtils";
    
    export let mId: string = "delete-exercise-modal";
    export let onYes: (exerciseId: string) => void = (d) => {};
    export let onNo: (exerciseId: string) => void = (d) => {};

    const FORM_SUFFIX = "-form";
    const EXERCISE_ID_SUFFIX = "-exercise-id";

    const FORM_NAME = mId + FORM_SUFFIX;
    const EXERCISE_ID_FIELD_NAME = mId + EXERCISE_ID_SUFFIX;

    function yes(modalId: string): void {
        let exerciseId: string = ModalUtils.getFormData(modalId, "exercise-id");
        onYes(exerciseId);
        ModalUtils.closeModal(modalId);
    }

    function no(modalId: string): void {
        let exerciseId: string = ModalUtils.getFormData(modalId, "exercise-id");
        onNo(exerciseId);
        ModalUtils.closeModal(modalId);
    }
</script>

<div id={mId} class="modal">
    <div class="modal-box w-11/12 max-w-5xl">
        <h1 class="font-semibold text-2xl uppercase pb-2">
            Delete exercise
        </h1>
        <form name={FORM_NAME} id={FORM_NAME}>
            <input type="hidden" name={EXERCISE_ID_FIELD_NAME} id={EXERCISE_ID_FIELD_NAME}/>
        </form>
        <p class="text-md px-1 py-2">
           Are you sure that you want to delete this exercise? 
        </p>
        <div class="flex">
            <div class="grow"></div>
            <div class="grow-0 modal-action">
                <button class="btn btn-success" on:click={() => yes(mId)}>Yes</button>
                <button class="btn btn-error" on:click={() => no(mId)}>No</button>
            </div>
        </div>
    </div>
</div>
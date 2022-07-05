<script lang="ts">
    import type { ExercisePayload } from "../util/types";
    import ModalUtils from "../util/ModalUtils";

    export let mId: string;
    export let caption: string = "&nbsp;";
    export let onOk: (data: ExercisePayload) => void = (d) => {};
    export let onCancel: (data: ExercisePayload) => void = (d) => {};

    const FORM_SUFFIX = "-form";
    const ID_SUFFIX = "-id";
    const NAME_SUFFIX = "-name";
    const DESCRIPTION_SUFFIX = "-description";
    const BODYWEIGHT_SUFFIX = "-bodyweight";

    const FORM_NAME = mId + FORM_SUFFIX;
    const ID_FIELD_NAME = mId + ID_SUFFIX;
    const NAME_FIELD_NAME = mId + NAME_SUFFIX;
    const DESCRIPTION_FIELD_NAME = mId + DESCRIPTION_SUFFIX;
    const BODYWEIGHT_FIELD_NAME = mId + BODYWEIGHT_SUFFIX;

    function ok(modalId: string): void {
        let data: ExercisePayload = {
            name: ModalUtils.getFormData(modalId, "name"),
            description: ModalUtils.getFormData(modalId, "description"),
            bodyweight: ModalUtils.getFormData(modalId, "bodyweight")
        };
        onOk(data);
        ModalUtils.closeModal(modalId);
    }

    function cancel(modalId: string): void {
        let data: ExercisePayload = {
            name: ModalUtils.getFormData(modalId, "name"),
            description: ModalUtils.getFormData(modalId, "description"),
            bodyweight: ModalUtils.getFormData(modalId, "bodyweight")
        };
        onCancel(data);
        ModalUtils.closeModal(modalId);
    }
</script>

<div id={mId} class="modal">
    <div class="modal-box w-11/12 max-w-5xl">
        <h1 class="font-semibold text-2xl uppercase">
            {caption}
        </h1>
        <form name={FORM_NAME} id={FORM_NAME}>
            <input type="hidden" name={ID_FIELD_NAME} id={ID_FIELD_NAME}/>
            <div class="form-control w-full max-w-xs">
                <label for={NAME_FIELD_NAME} class="label">
                    <span class="label-text">Name</span>
                </label>
                <input type="text" placeholder="Name" name={NAME_FIELD_NAME} id={NAME_FIELD_NAME} class="input input-bordered input-primary w-full max-w-xs" required/>
            </div>
            <div class="form-control w-full max-w-xs">
                <label for={DESCRIPTION_FIELD_NAME} class="label">
                    <span class="label-text">Description</span>
                </label>
                <textarea placeholder="Description" form={FORM_NAME} name={DESCRIPTION_FIELD_NAME} id={DESCRIPTION_FIELD_NAME} class="textarea textarea-primary"/>
            </div>
            <div class="form-control w-full max-w-xs">
                <label for={BODYWEIGHT_FIELD_NAME} class="label cursor-pointer">
                    <span class="label-text">Bodyweight exercise</span>
                    <input type="checkbox" name={BODYWEIGHT_FIELD_NAME} id={BODYWEIGHT_FIELD_NAME} class="checkbox checkbox-primary"/>
                </label>
            </div>
        </form>
        <div class="flex">
            <div class="grow"/>
            <div class="grow-0 modal-action">
                <button class="btn btn-success" on:click={() => ok(mId)}>Ok</button>
                <button class="btn btn-error" on:click={() => cancel(mId)}>Cancel</button>
            </div>
        </div>
    </div>
</div>
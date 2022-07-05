class ModalUtils {
    static MODAL_OPEN_CLASS = "modal-open";

    static openModal(modalId: string): void {
        let modalElement = document.querySelector('#' + modalId);
        let classList = modalElement.classList;
        if(classList.contains(ModalUtils.MODAL_OPEN_CLASS)) {
            return;
        }
        classList.add(ModalUtils.MODAL_OPEN_CLASS);
    }

    static closeModal(modalId: string): void {
        let modalElement = document.querySelector('#' + modalId);
        let classList = modalElement.classList;
        if(classList.contains(ModalUtils.MODAL_OPEN_CLASS)) {
            classList.remove(ModalUtils.MODAL_OPEN_CLASS);
        }
    }

    static getFormData(modalId: string, field: string): any {
        let form: HTMLFormElement = document.forms[modalId + "-form"];
        let formElement = form[modalId + '-' + field];
        if(formElement.type == "checkbox") {
            return formElement.checked ? true : false;
        }
        return formElement.value;
    }

    static setFormData(modalId: string, field: string, data: any): void {
        let form: HTMLFormElement = document.forms[modalId + "-form"];
        let formElement = form[modalId + '-' + field];
        if(formElement.type == "checkbox") {
            formElement.checked = data ? true : false;
        }
        formElement.value = data;
    }
};

export default ModalUtils;
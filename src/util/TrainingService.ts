import { invoke } from "@tauri-apps/api/tauri";
import type { Training, TrainingPayload } from "./types";

class TrainingService {

    static GET_TRAININGS_COMMAND = "get_trainings";
    static GET_TRAINING_COMMAND = "get_training";
    static ADD_TRAINING_COMMAND = "add_training";
    static UPDATE_TRAINING_COMMAND = "update_training";

    async getTrainings(): Promise<Training[]> {
        return await invoke(TrainingService.GET_TRAININGS_COMMAND);
    }

    async getTraining(trainingId: string): Promise<Training> {
        return await invoke(TrainingService.GET_TRAINING_COMMAND, {id: trainingId});
    }

    async addTraining(trainingPayload: TrainingPayload): Promise<void> {
        await invoke(TrainingService.ADD_TRAINING_COMMAND, {training: trainingPayload});
    }

    async updateTraining(trainingId: string, trainingPayload: TrainingPayload): Promise<void> {
        await invoke(TrainingService.UPDATE_TRAINING_COMMAND, {id: trainingId, training: trainingPayload});
    }
}

export default TrainingService;
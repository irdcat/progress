import { invoke } from "@tauri-apps/api/tauri";
import type { Exercise, ExercisePayload } from "./types";

class ExerciseFacade {

    static GET_EXERCISES_COMMAND = "get_exercises";
    static GET_EXERCISE_COMMAND = "get_exercise";
    static ADD_EXERCISE_COMMAND = "add_exercise";
    static UPDATE_EXERCISE_COMMAND = "update_exercise";

    async getExercises(): Promise<Exercise[]> {
        return await invoke(ExerciseFacade.GET_EXERCISES_COMMAND);
    }

    async getExercise(exerciseId: string): Promise<Exercise> {
        return await invoke(ExerciseFacade.GET_EXERCISE_COMMAND, {id: exerciseId});
    }

    async addExercise(exercisePayload: ExercisePayload): Promise<void> {
        await invoke(ExerciseFacade.ADD_EXERCISE_COMMAND, {exercise: exercisePayload});
    }

    async updateExercise(exerciseId: string, exercisePayload: ExercisePayload): Promise<void> {
        await invoke(ExerciseFacade.UPDATE_EXERCISE_COMMAND, {id: exerciseId, exercise: exercisePayload});
    }
};

export default ExerciseFacade;
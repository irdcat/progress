import { invoke } from "@tauri-apps/api/tauri";
import type { Exercise, ExercisePayload } from "./types";

const GET_EXERCISES_COMMAND = "get_exercises";
const GET_EXERCISE_COMMAND = "get_exercise";
const ADD_EXERCISE_COMMAND = "add_exercise";
const UPDATE_EXERCISE_COMMAND = "update_exercise";

class ExerciseFacade {

    async getExercises(): Promise<Exercise[]> {
        return await invoke(GET_EXERCISES_COMMAND);
    }

    async getExercise(exerciseId: string) : Promise<Exercise> {
        return await invoke(GET_EXERCISE_COMMAND, {id: exerciseId});
    }

    async addExercise(exercisePayload: ExercisePayload) : Promise<void> {
        console.log(exercisePayload);
        await invoke(ADD_EXERCISE_COMMAND, {exercise: exercisePayload});
    }

    async updateExercise(exerciseId: string, exercisePayload: ExercisePayload) : Promise<void> {
        await invoke(UPDATE_EXERCISE_COMMAND, {id: exerciseId, exercise: exercisePayload});
    }
};

export default ExerciseFacade;
import { invoke } from "@tauri-apps/api/tauri";
import type { ExerciseDetails } from "./types";

class ExerciseDetailsService {

    static GET_EXERCISE_DETAILS_COMMAND = "get_exercise_details";

    async getExerciseDetails(exerciseId: string): Promise<ExerciseDetails[]> {
        return await invoke(ExerciseDetailsService.GET_EXERCISE_DETAILS_COMMAND, {exerciseId: exerciseId});
    }
};

export default ExerciseDetailsService;

type Exercise = {
    id: string,
    name: string,
    description: string,
    bodyweight: boolean
};

type ExercisePayload = {
    name: string,
    description: string,
    bodyweight: boolean
};

export type {
    Exercise, ExercisePayload
}
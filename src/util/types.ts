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

type TrainingSet = {
    id: string,
    repetitions: number,
    weight: number
};

type TrainingEntry = {
    id: string,
    exercise_id: string,
    sets: TrainingSet[]
};

type Training = {
    id: string,
    date: string,
    entries: TrainingEntry[]
};

type TrainingPayload = {
    date: string,
    entries: TrainingEntry[]
};

type ExerciseDetails = {
    repetitions: number,
    weight: number,
    date: string
};

export type {
    Exercise, 
    ExercisePayload,
    TrainingSet, TrainingEntry, Training,
    TrainingPayload,
    ExerciseDetails
}
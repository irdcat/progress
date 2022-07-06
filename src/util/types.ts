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
    exerciseId: string,
    sets: TrainingSet[]
};

type Training = {
    id: string,
    date: Date,
    exerciseEntries: TrainingEntry[]
};

type TrainingPayload = {
    date: Date,
    exerciseEntries: TrainingEntry[]
};

export type {
    Exercise, 
    ExercisePayload,
    TrainingSet, TrainingEntry, Training,
    TrainingPayload
}
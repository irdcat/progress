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

type TrainingSetPayload = {
    repetitions: number,
    weight: number
}

type TrainingEntry = {
    id: string,
    exerciseId: string,
    sets: TrainingSet[]
};

type TrainingEntryPayload = {
    exerciseId: string,
    sets: TrainingSetPayload[]
};

type Training = {
    id: string,
    date: Date,
    exerciseEntries: TrainingEntry[]
};

type TrainingPayload = {
    date: Date,
    exerciseEntries: TrainingEntryPayload[]
};

export type {
    Exercise, 
    ExercisePayload,
    TrainingSet, TrainingEntry, Training,
    TrainingSetPayload, TrainingEntryPayload, TrainingPayload
}
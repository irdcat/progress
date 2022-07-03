<script lang="ts">
    function formatDate(date: Date): string {
        return date.toISOString().substring(0, 10);
    }

    //TODO: Fetch trainings from backend
    let trainings = [
        {id: "1", date: formatDate(new Date(2022, 5, 14))},
        {id: "2", date: formatDate(new Date(2022, 5, 16))},
        {id: "3", date: formatDate(new Date(2022, 5, 18))},
        {id: "4", date: formatDate(new Date(2022, 5, 21))},
        {id: "5", date: formatDate(new Date(2022, 5, 23))}
    ];

    function openTrainingAddModal(): void {
        // TODO: Implement later
    }

    function openTrainingEditModal(id: string): void {
        // TODO: Implement later
    }

    function goToTrainingDetails(id: string): void {
        // TODO: Implement later
    }
</script>

<div class="w-full flex flex-col">
    <div class="flex grow p-2 bg-base-300">
        <p class="grow font-semibold text-2xl leading-8 uppercase">
            Trainings
        </p>
        <div class="grow-0">
            <button class="btn btn-primary btn-sm" on:click={() => openTrainingAddModal()}>Add</button>
        </div>
    </div>
    {#each trainings as {id, date}, index (id)}
        <div class="flex grow p-2">
            <p class="grow text-md font-normal">
                {date}
            </p>
            <div class="grow-0">
                <button class="btn btn-secondary btn-sm" on:click={() => openTrainingEditModal(id)}>Edit</button>
                <button class="btn btn-secondary btn-sm" on:click={() => goToTrainingDetails(id)}>Details</button>
            </div>
        </div>
    {/each}
</div>
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

<table class="table table-compact w-full">
    <thead>
        <tr>
            <th>Trainings</th>
            <th class="w-14">
            </th>
            <th class="w-14">
                <button class="btn btn-primary btn-xs" on:click={() => openTrainingAddModal()}>Add</button>
            </th>
        </tr>
    </thead>
    <tbody>
        {#each trainings as {id, date}, index (id)}
            <tr>
                <td>{date}</td>
                <td colspan="2" class="w-14">
                    <button class="btn btn-secondary btn-xs" on:click={() => openTrainingEditModal(id)}>Edit</button>
                    <button class="btn btn-secondary btn-xs" on:click={() => goToTrainingDetails(id)}>Details</button>
                </td>
            </tr>
        {/each}
    </tbody>
</table>
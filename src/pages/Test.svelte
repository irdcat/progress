<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    
    let testData = [];

    function addNewTestData() {
        const ADD_NEW_TEST_DATA_COMMAND = "create_test_data";
        let nameInput: HTMLInputElement = document.querySelector("input#name");
        invoke(ADD_NEW_TEST_DATA_COMMAND, {name: nameInput.value}).then((data) => {
            testData = data;
            console.log(`Returned data ${data}`);
            console.log(testData);
        });
    }
</script>

<label for="name">Name</label>
<input type="text" class="input input-bordered input-primary w-full max-w-xs" id="name"/>
<button class="btn btn-primary" on:click={() => addNewTestData()}>Add</button>
<div>
    {#each testData as data}
        <p>{data.name}</p>
    {/each}
</div>

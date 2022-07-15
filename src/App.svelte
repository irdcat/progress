<script lang="ts">
    import Router, { location } from "svelte-spa-router";
    import { onDestroy } from "svelte";
    import Home from "./pages/Home.svelte";
    import Exercises from "./pages/Exercises.svelte";
    import ExerciseDetails from "./pages/ExerciseDetails.svelte";
    import Trainings from "./pages/Trainings.svelte";
    import TrainingDetails from "./pages/TrainingDetails.svelte";
    import { appWindow } from '@tauri-apps/api/window'
    
    const routes = {
        "/": Home,
        "/exercises": Exercises,
        "/exercises/details": ExerciseDetails,
        "/trainings": Trainings,
        "/trainings/details": TrainingDetails
    };

    let locationParts = $location.split("/").filter((part) => part != "");
    let locationPartsFormatted = locationParts.map((part) => part.charAt(0).toUpperCase() + part.slice(1));

    const unsubscribeLocation = location.subscribe(value => {
        locationParts = value.split("/").filter((part) => part != "");
        locationPartsFormatted = locationParts.map((part) => part.charAt(0).toUpperCase() + part.slice(1));
    });

    function assembleLocationUrl(index) : string {
        let url = "#";
        for(let i = 0; i <= index; i++) {
            url += "/" + locationParts[index];
        }
        return url;
    }

    onDestroy(unsubscribeLocation);
</script>

<main class="h-screen w-screen overflow-hidden select-none">
    <div class="navbar h-8 min-h-8 bg-slate-100 m-auto text-black flex pr-0"
        on:mousedown={async (_) => await appWindow.startDragging()}>
        <div class="grow text-sm breadcrumbs">
            <ul>
                {#if !locationParts.length}
                    <li class="text-md normal-case font-semibold">Progress</li>
                {:else}
                    <li class="text-md normal-case font-semibold"><a href="#/">Progress</a></li>
                {/if}
                {#each locationPartsFormatted as formattedLocation, index}
                    {#if index == locationParts.length - 1}
                        <li>{formattedLocation}</li>
                    {:else}
                        <li><a href={assembleLocationUrl(index)}>{formattedLocation}</a></li>    
                    {/if}
                {/each}
            </ul>            
        </div>
        <div id="sysmenu" class="grow-0">
            <div id="sysmenu-minimize" on:click={async () => await appWindow.minimize()}
                class="inline-flex justify-center items-center w-9 h-8 hover:bg-gray-400">
                <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize"/>
            </div>
            <div id="sysmenu-maximize" on:click={async () => await appWindow.toggleMaximize()}
                class="inline-flex justify-center items-center w-9 h-8 hover:bg-gray-400">
                <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize"/>
            </div>
            <div id="sysmenu-close" on:click={async () => await appWindow.close()}
                class="inline-flex justify-center items-center w-9 h-8 hover:bg-red-400">
                <img src="https://api.iconify.design/mdi:close.svg" alt="close"/>
            </div>
        </div>
    </div>
    <div class="m-auto w-full overflow-y-auto h-[calc(100%-2rem)]">
        <Router {routes}/>
    </div>
</main>

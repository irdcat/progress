<script lang="ts">
    import Router, { location } from "svelte-spa-router";
    import { onDestroy } from "svelte";
    import Home from "./pages/Home.svelte";
    import Exercises from "./pages/Exercises.svelte";
    import ExerciseDetails from "./pages/ExerciseDetails.svelte";
import { empty } from "svelte/internal";
    
    const routes = {
        "/": Home,
        "/exercises": Exercises,
        "/exercises/details": ExerciseDetails
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

<main>
    <div class="navbar h-8 min-h-8 bg-base-300 max-w-5xl m-auto">
        <div class="text-sm breadcrumbs">
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
    </div>
    <div class="m-auto max-w-5xl">
        <Router {routes}/>
    </div>
</main>
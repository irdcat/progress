<script lang="ts">
    import Router from "svelte-spa-router";

    import Home from "./pages/Home.svelte";
    import Exercises from "./pages/Exercises.svelte";
    import ExerciseDetails from "./pages/ExerciseDetails.svelte";

    const routes = {
        "/": Home,
        "/exercises": Exercises,
        "/exercises/:id/details": ExerciseDetails
    };
</script>

<main>
    <div class="m-auto max-w-5xl">
        <Router {routes}/>
    </div>
</main>
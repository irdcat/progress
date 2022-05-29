<script lang="ts">
    import Router from "svelte-spa-router";

    import Home from "./pages/Home.svelte";
    import Exercises from "./pages/Exercises.svelte";

    const routes = {
        "/": Home,
        "/exercises": Exercises
    };
</script>

<main>
    <Router {routes}/>
</main>
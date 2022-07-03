<script lang="ts">
    import { LinkedChart, LinkedLabel, LinkedValue } from "svelte-tiny-linked-charts";
    import { querystring } from "svelte-spa-router";
    import { parse } from "qs";

    export let params = parse($querystring)

    function generateFakeData(times, min, max) {
        const data = {};
        const date = new Date("2022-03-01T00:00:00Z");

        for(let i=0; i<times; i++) {
            const setDate = date.setDate(date.getDate() + 1);
            const formattedDate = new Date(setDate).toISOString().substring(0, 10);

            data[formattedDate] = Math.floor(Math.random() * (max - min)) + min;
        }
        return data;
    }

    let volumes = generateFakeData(50, 1000, 3000);
    let avgVolumes = {};
    for(const [key, value] of Object.entries(volumes)) {
        avgVolumes[key] = Math.floor(value / 3);
    }
    let intensities = generateFakeData(50, 30, 100);
</script>

<div class="w-full flex flex-col">
    <div class="flex grow p-2 bg-base-300">
        <p class="grow font-semibold text-xl leading-8 uppercase">
            Exercise {params.id}
        </p>
        <p class="grow-0 w-28 font-semibold text-xl leading-8 uppercase text-left">
            Date
        </p>
        <p class="grow-0 w-28 font-semibold text-xl leading-8 uppercase text-left">
            Value
        </p>
    </div>
    <div class="flex grow p-2 bg-base-100">
        <p class="grow font-semibold text-lg leading-8 uppercase">
            Volume
        </p>
    </div>
    <div class="flex grow p-2 items-end">
        <div class="grow">
            <LinkedChart data={volumes} width="700" grow="true" height="100" align="left" fill="#20ff20" linked="link" uid="volume"/>
        </div>
        <p class="grow-0 w-28 text-left font-medium">
            <LinkedLabel linked="link"/>
        </p>
        <p class="grow-0 w-28 text-left font-medium">
            <LinkedValue uid="volume"/>
        </p>
    </div>
    <div class="flex grow p-2 bg-base-100">
        <p class="grow font-semibold text-lg leading-8 uppercase">
            Average volume per set
        </p>
    </div>
    <div class="flex grow p-2 items-end">
        <div class="grow">
            <LinkedChart data={avgVolumes} width="700" grow="true" height="100" align="left" fill="#50ff50" linked="link" uid="avgVolume"/>
        </div>
        <p class="grow-0 w-28 text-left font-medium">
            <LinkedLabel linked="link"/>
        </p>
        <p class="grow-0 w-28 text-left font-medium">
            <LinkedValue uid="avgVolume"/>
        </p>
    </div>
    <div class="flex grow p-2 bg-base-100">
        <p class="grow font-semibold text-lg leading-8 uppercase">
            Intensity
        </p>
    </div>
    <div class="flex grow p-2 items-end">
        <div class="grow">
            <LinkedChart data={intensities} width="700" grow="true" height="100" align="left" fill="#ff2020" linked="link" uid="intensity"/>
        </div>
        <p class="grow-0 w-28 text-left font-medium">
            <LinkedLabel linked="link"/>
        </p>
        <p class="grow-0 w-28 text-left font-medium">
            <LinkedValue uid="intensity"/>
        </p>
    </div>
</div>
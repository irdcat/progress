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

<table class="table w-full">
    <thead>
        <tr>
            <th colspan="3">Exercise {params.id}</th>
            <th class="text-center"><LinkedLabel linked="link" empty="Value"/></th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td colspan="5" class="font-semibold">Volume</td>
        </tr>
        <tr>
            <td colspan="3">
                <LinkedChart data={volumes} width="700" grow="true" height="100" align="left" fill="#20ff20" linked="link" uid="volume"/>
            </td>
            <td class="text-center">
                <LinkedValue uid="volume"/>
            </td>
        </tr>
        <tr>
            <td colspan="5" class="font-semibold">Average volume per set</td>
        </tr>
        <tr>
            <td colspan="3">
                <LinkedChart data={avgVolumes} width="700" grow="true" height="100" align="left" fill="#50ff50" linked="link" uid="avgVolume"/>
            </td>
            <td class="text-center">
                <LinkedValue uid="avgVolume"/>
            </td>
        </tr>
        <tr>
            <td colspan="5" class="font-semibold">Intensity</td>
        </tr>
        <tr>
            <td colspan="3">
                <LinkedChart data={intensities} width="700" grow="true" height="100" align="left" fill="#ff2020" linked="link" uid="intensity"/>
            </td>
            <td class="text-center">
                <LinkedValue uid="intensity"/>
            </td>
        </tr>
    </tbody>
</table>
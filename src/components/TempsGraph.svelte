<script lang="ts">
    import { Line } from "svelte-chartjs";
    import { type ITempTimestamp } from "$lib";
    import {
        Chart as ChartJS,
        Title,
        Tooltip,
        Legend,
        LineElement,
        LinearScale,
        PointElement,
        CategoryScale,
    } from "chart.js";
    import type { ChartData, Point } from "chart.js";
    import { onMount } from "svelte";

    ChartJS.register(
        Title,
        Tooltip,
        Legend,
        LineElement,
        LinearScale,
        PointElement,
        CategoryScale,
    );

    let chartRef: ChartJS<"line", (number | Point)[], unknown>;

    export let timeline: { [device: string]: ITempTimestamp[] };

    let energy = [65, 59, 80, 81, 56, 90];

    const data: ChartData<"line", (number | Point)[], unknown> = {
        labels: ["January", "February", "March", "April", "May", "Test"],
        datasets: [
            {
                label: "My First dataset",
                fill: false,
                backgroundColor: "rgba(225, 204,230, .3)",
                borderColor: "rgb(205, 130, 158)",
                borderCapStyle: "round",
                borderDash: [],
                pointBorderColor: "rgb(205, 130,1 58)",
                pointBackgroundColor: "rgb(255, 255, 255)",
                pointHoverRadius: 5,
                pointHoverBackgroundColor: "rgb(0, 0, 0)",
                pointRadius: 5,
                pointHitRadius: 10,
                data: energy,
            },
        ],
    };

    onMount(() => {
        const interval = setInterval(() => {
            rendom();
        }, 1000);
        return () => {
            clearInterval(interval);
        };
    });

    function rendom() {
        let index = Math.round(Math.random() * 5);
        energy[index] = Math.round(Math.random() * 100);
        chartRef.update();
    }
</script>

<Line
    bind:chart={chartRef}
    {data}
    height={300}
    width={300}
    options={{ maintainAspectRatio: true, responsive: false }}
/>

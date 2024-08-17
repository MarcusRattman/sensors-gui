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
    import { timelinestore } from "../stores/";

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

    let timestamps = ["xx:xx::xx"];
    let temps = [0, 0, 0, 0, 0, 0];
    let energy = [0, 0, 0, 0, 0, 0];

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

    // timelinestore.subscribe((newTimeline) => {
    //     timestamps = newTimeline["Tctl"].map((t) => t.timestamp);
    //     temps = newTimeline["Tctl"].map((t) => t.temp);
    //     chartRef.update();
    // });

    onMount(() => {
        const interval = setInterval(() => {
            rendom();
        }, 2000);
        return () => {
            clearInterval(interval);
        };
    });

    function rendom() {
        data.datasets[0].data = $timelinestore["Tctl"].map((t) => t.temp);
        data.labels = $timelinestore["Tctl"].map((t) => t.timestamp);
        chartRef.update();
    }
</script>

<Line
    bind:chart={chartRef}
    {data}
    height={300}
    width={300}
    options={{
        maintainAspectRatio: true,
        responsive: false,
        scales: { y: { min: 30, max: 100 } },
        animation: { duration: 0 },
    }}
/>

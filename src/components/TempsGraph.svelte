<script lang="ts">
    import { Line } from "svelte-chartjs";
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

    const data: ChartData<"line", (number | Point)[], unknown> = {
        labels: ["xxx"],
        datasets: [
            {
                label: "My First dataset",
                fill: false,
                backgroundColor: "rgba(225, 204,230, .3)",
                borderColor: "rgb(205, 130, 158)",
                borderCapStyle: "round",
                borderDash: [],
                pointBorderColor: "black",
                pointBackgroundColor: "rgb(255, 255, 255)",
                pointHoverBackgroundColor: "rgb(0, 0, 0)",
                pointRadius: 3,
                pointHitRadius: 5,
                data: [0],
            },
        ],
    };

    timelinestore.subscribe(refresh);

    async function refresh() {
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
        maintainAspectRatio: false,
        responsive: false,
        scales: { y: { min: 30, max: 100 }, x: { display: false } },
        animation: { duration: 0 },
    }}
/>

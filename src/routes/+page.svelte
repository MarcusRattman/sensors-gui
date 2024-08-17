<script lang="ts">
  import { type ITempTimestamp, temp_timestamp } from "$lib";
  import { invoke } from "@tauri-apps/api/tauri";
  import { TempsTable, TempsGraph } from "../components";
  import { timelinestore } from "../stores";

  let temps: { [device: string]: number } = {};
  let min: { [device: string]: number } = {};
  let max: { [device: string]: number } = {};
  let timeline: { [device: string]: ITempTimestamp[] } = {};

  setInterval(update_temps, 1000);

  async function update_temps() {
    temps = await invoke("read_temps");

    for (let [device, temp] of Object.entries(temps)) {
      if (!min[device] && !max[device]) {
        min[device] = temp;
        max[device] = temp;
      }

      if (temp < min[device]) {
        min[device] = temp;
      }

      if (temp > max[device]) {
        max[device] = temp;
      }

      if (!timeline[device]) {
        timeline[device] = [];
      }

      timeline[device].push(temp_timestamp(temp));

      if (timeline[device].length > 100) {
        timeline[device].shift();
      }

      timelinestore.set(timeline);
    }
  }

  update_temps();
</script>

<div class="container">
  <div>
    <TempsTable {temps} {min} {max} />
  </div>

  <div>
    <TempsGraph />
  </div>
</div>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    display: grid;
    grid-template-columns: 1fr 1fr;
    place-items: center;
    gap: 1rem;
  }

  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  button {
    outline: none;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>

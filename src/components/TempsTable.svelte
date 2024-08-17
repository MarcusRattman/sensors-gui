<script lang="ts">
  export let temps: { [key: string]: number } = {};
  export let min: { [key: string]: number } = {};
  export let max: { [key: string]: number } = {};
</script>

<table>
  <thead>
    <tr>
      <th>Device</th>
      <th>Temp</th>
      <th>Min</th>
      <th>Max</th>
    </tr>
  </thead>
  <tbody>
    {#if temps}
      {#each Object.entries(temps) as [k, v]}
        <tr>
          <td>{k}</td>
          <td>{v}</td>
          <td>{min[k]}</td>
          <td>{max[k]}</td>
        </tr>
      {/each}
    {/if}
  </tbody>
</table>

<style scoped>
  table {
    width: 300px;
    border: 1px solid #0f0f0f;
    border-collapse: collapse;
  }

  th,
  td {
    padding: 0.8em 1em;
    text-align: left;
  }

  th {
    background-color: #222;
  }

  tr {
    &:nth-child(even) {
      background-color: #333;
    }
  }
</style>

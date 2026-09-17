<script lang="ts">
  import type { WidgetManager } from "../functional/keys";

  let { wM = $bindable() }: { wM: WidgetManager } = $props();


  let commands: string[][] = $state([]);

  function createCommand(cmd: string, helper: string) {
    commands.push([cmd, helper]);
  }

  createCommand("help", "Opens the help dialog.");
  // createCommand("check timings", "Checks how long it takes for endpoints to respond.");
  // createCommand("success rates", "Check endpoint success rate over a given time.");
  createCommand("shuffle colours", "Shuffle the colours of the map layers.");
  createCommand("enable borders", "Enable tile borders.");
  createCommand("disable borders", "Disable tile borders.");
  createCommand("clear", "Clears the map of all items.");
  createCommand("history", "Opens up the layer history.");
</script>

<div class="help-div">
  <table>
    <thead>
      <tr>
        <th>Command</th>
        <th>Purpose</th>
      </tr>
    </thead>
    <tbody>
    {#each commands as [c, k]}
      <tr>
        <td>{c}</td>
        <td>{k}</td>
      </tr>
    {/each}
    </tbody>
  </table>
  <div
    class="close-button"
    role="button"
    onkeydown={(ev) => {
      if (ev.key === "Enter") {
        wM.toggleHelp();
      }
    }}
    tabindex="0"
    onclick={() => wM.toggleHelp()}
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
      class="lucide lucide-x"
      ><path d="M18 6 6 18" /><path d="m6 6 12 12" /></svg
    >
  </div>
</div>

<style>
  .help-div {
    display: flex;
    flex-direction: column;
    border-radius: 12px;
    background-color: #2c3036;
    z-index: 10;
    position: fixed;
    top: 50vh;
    left: 50vw;
    transform: translate(-50%, -50%);
    color: white;
    padding: 15px;
    max-width: 80vw;
  }
  .help-div table {
    border-collapse: collapse;
    border-style: hidden;
  }

  .help-div td,
  .help-div th {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    border: solid white 1px;
    padding-right: 5px;
    padding-left: 5px;
  }
  .close-button {
    position: fixed;
    color: white;
    top: 12px;
    right: 12px;
    cursor: pointer;
  }
</style>

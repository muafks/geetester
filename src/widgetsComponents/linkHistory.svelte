<script lang="ts">
  import { onMount } from "svelte";
  import type { WidgetManager } from "../functional/keys";
  import { HistoryManager } from "../functional/history";
  import { invoke } from "@tauri-apps/api/core";

  let { wM }: { wM: WidgetManager } = $props();

  let histories: string[] = $state([]);

  onMount(async () => {
    const histMan = new HistoryManager();
    histories = await histMan.getEntries();
  });

  function close() {
    wM.toggleHistory();
  }

  function addToMapAndClose(url: string) {
    invoke("add_link", { url: url });
    close();
  }
</script>

<div class="bg">
  <div
    class="exit"
    tabindex="0"
    role="button"
    style="cursor: pointer;"
    onkeydown={(ev) => {
      if (ev.key === "Enter") {
        close();
      }
    }}
    onclick={() => {
      close();
    }}
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
  <h1 class="tittle-let">Endpoint History</h1>
  <div class="histories">
    {#each histories as history}
      <div class="history-row">
        <p>{history}</p>
        <div class="vert-sep"></div>
        <button
          onclick={() => {
            addToMapAndClose(history);
          }}>Add to map</button
        >
      </div>
    {/each}
  </div>
</div>

<style>
.vert-sep {
    height: 20px;
    border: solid white 1px;
}
button {
    height: 100%;
}
  .bg {
    display: flex;
    flex-direction: column;
    position: fixed;
    left: 50vw;
    top: 50vh;
    transform: translate(-50%, -50%);
    border-radius: 12px;
    z-index: 10;
    background-color: #2c3036;
    min-width: 500px;
    color: white;
  }

  .tittle-let {
    padding-left: 12px;
    
  }

  .exit {
    position: fixed;
    top: 12px;
    right: 12px;
  }

  .histories {
    align-self: center;
    display: flex;
    flex-direction: column;
    width: 90%;
    align-items: center;
    max-height: 60vh;
    overflow-y: scroll;
    max-width: 50vw;
    overflow-x: hidden;
  }

  .history-row {
    display: flex;
    flex-direction: row;
    gap: 25px;
    align-items: center;
    justify-content: center;
    overflow-x: hidden;
  }

  .history-row p {
    text-overflow: ellipsis;
  }
  .history-row button {
    cursor: pointer;
  }
</style>

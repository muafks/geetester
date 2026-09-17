<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    commandKeyring,
    type CommandsInKeyring,
    type WidgetManager,
  } from "../functional/keys";
  import * as maplibregl from "maplibre-gl";
  import { getContext, onMount } from "svelte";
  import { HistoryManager } from "../functional/history";
  let {
    wM = $bindable(),
    map = $bindable(),
  }: { wM: WidgetManager; map: maplibregl.Map | null } = $props();
  let searchInput: HTMLInputElement | null = $state(null);
  const availableTools: CommandsInKeyring = getContext(commandKeyring);
  let historyMan: HistoryManager|null = $state(null);
  
  onMount(() => {
    historyMan = new HistoryManager();
  });

  function processSearch() {
    if (!searchInput||!historyMan) {
      return;
    }

    // match one-offs like help

    switch (searchInput.value.toLowerCase()) {
      case "help":
        wM.toggleHelp();
        clearSearchInput();
        break;
      case "check timings":
        clearSearchInput();
        break;
      case "success rates":
        clearSearchInput();
        break;
      case "shuffle colors":
      case "shuffle colours":
        availableTools.sc();
        clearSearchInput();
        break;
      case "enable borders":
        if (map) {
          map.showTileBoundaries = true;
        }
        clearSearchInput();
        break;
      case "disable borders":
        if (map) {
          map.showTileBoundaries = false;
        }
        clearSearchInput();
        break;
      case "clear":
        availableTools.clas(true);
        clearSearchInput();
        break;
      case "history":
        wM.toggleHistory();
        clearSearchInput();
        break;
      default:
        break;
    }

    if (!URL.canParse(searchInput.value)) return;
    console.log("adding link");
    invoke("add_link", { url: searchInput.value });
    historyMan.createEntry(searchInput.value);
    clearSearchInput();
  }
  function clearSearchInput() {
    if (!searchInput) {
      return;
    }
    searchInput.value = "";
  }
</script>

<div class="bar">
  <input
    bind:this={searchInput}
    type="text"
    class="input-field"
    placeholder="Paste a link or type 'help' for commands."
    onkeydown={(ev) => {
      if (ev.key === "Enter") {
        processSearch();
      }
    }}
  />
</div>

<style>
  .bar {
    min-height: 50px;
    height: 50px;
    background-color: #2c3036;
    width: 100%;
    border-radius: 12px;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .input-field {
    width: 100%;
    height: 100%;
    background-color: transparent;
    color: white;
    border: transparent;
    box-sizing: border-box;
    padding-left: 12px;
  }
</style>

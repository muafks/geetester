<script lang="ts">
  import * as maplibregl from "maplibre-gl";
  import { getContext } from "svelte";
  import { commandKeyring, type CommandsInKeyring } from "../functional/keys";
  let {
    layers = $bindable(),
    map = $bindable(),
  }: { layers?: string[]; map: maplibregl.Map | null } = $props();

  let layer_ready_stats = $derived(
    layers?.map((v) => getLayerPropertiesFromStr(v)),
  );

  const availableTools: CommandsInKeyring = getContext(commandKeyring);

  let bcp: HTMLInputElement | null = $state(null);

  function getLayerPropertiesFromStr(
    ls: string,
  ): { color: string; title: string, l_type: string } | null {
    if (!map) return null;

    const mLayer = map.getLayer(ls) as
      | maplibregl.LayerSpecification
      | undefined;
    if (!mLayer) return null;

    const layerId = mLayer.id;
    let rawColor: unknown;

    switch (mLayer.type) {
      case "fill":
        rawColor = map.getPaintProperty(layerId, "fill-color");
        break;
      case "line":
        rawColor = map.getPaintProperty(layerId, "line-color");
        break;
      case "circle":
        rawColor = map.getPaintProperty(layerId, "circle-color");
        break;
      default:
        return { color: "#FFF", title: layerId, l_type: mLayer.type };
    }

    const finalColor = typeof rawColor === "string" ? rawColor : "#FFF";

    return {
      color: finalColor,
      title: layerId,
      l_type: mLayer.type
    };
  }
</script>

<div class="explorer" class:none={!(layers && layers.length > 0)}>
  {#if layers && layers.length > 0}
    <h1>Active layers</h1>
    <div class="layers-list">
      {#each layer_ready_stats as lrs}
        <div class="layer-bg">
          <div
            class="layer-bg-color"
            style:background-color={lrs?.color}
            onclick={() => {bcp?.showPicker();}}
            onkeydown={(ev) => {
              if (ev.key === "Enter") {
                ev.preventDefault();
                bcp?.showPicker();
              }
            }}
            tabindex="0"
            role="button"
            aria-label="Layer's background color"
          ></div>

          <input bind:this={bcp} type="color" style="display: none;" onchange={() => {availableTools.scol(lrs?.title, lrs?.l_type, bcp?.value)}}/>
          <p>{lrs?.title}</p>
          <div
            onclick={() => {
              availableTools.ral(lrs!.title);
            }}
            tabindex="0"
            role="button"
            onkeypress={(ev) => {
              if (ev.key === "Enter") {
                availableTools.ral(lrs!.title);
              }
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
        </div>
      {/each}
    </div>
  {:else}
    <div>
      <h1>No layers to see</h1>
      <p>Try using the search bar to add one.</p>
    </div>
  {/if}
</div>

<style>
  .explorer {
    display: flex;
    flex-direction: column;
    flex: 1 1 0%;
    min-height: 0;
    width: 100%;
    background-color: #2c3036;
    border-radius: 8px;
    transition: all ease-in-out;
    transition-duration: 250ms;
    color: white;
    text-align: center;
    align-items: center;
    gap: 20px;
  }
  .explorer.none {
    align-items: center;
    justify-content: center;
  }
  .layers-list {
    width: 100%;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding-bottom: 12px;
  }
  .layer-bg {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    font-size: large;
    background-color: #2e343d;
    width: 80%;
    border-radius: 12px;
    overflow: scroll;
    flex-shrink: 0;
    gap: 10px;
  }
  .layer-bg div {
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
  }
  .layer-bg-color {
    display: flex;
    border-radius: 50%;
    height: 24px;
    width: 24px;
    transition: background-color ease-in-out 300ms;
  }
</style>

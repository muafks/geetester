<script lang="ts">
  import * as maplibregl from 'maplibre-gl';
  import type { WidgetManager } from "../functional/keys";
  import LayerExplorer from "./layerExplorer.svelte";
  import PropertiesExplorer from "./propertiesExplorer.svelte";
  import SearchWidget from "./searchWidget.svelte";

  let opened = $state(false);
  let { wM = $bindable(), map = $bindable(), layers = $bindable() }: { wM: WidgetManager, map: maplibregl.Map|null, layers: string[] } = $props();
  let propertiesExplorerOpen = $state(false);
  let propertiesExplorerData: any = $state(undefined);

  $effect(() => {
    wM.onUpdate = () => {
      propertiesExplorerOpen = wM.propertiesExplorerOpen;
      propertiesExplorerData = wM.propertiesExplorerData;
    };

    return () => {
      wM.onUpdate = undefined;
    };
  });

</script>

<div class="outer-container" class:out={opened}>
  <div class="drawer">
    <SearchWidget bind:wM={wM} bind:map={map}></SearchWidget>
    <LayerExplorer bind:map={map} bind:layers={layers}></LayerExplorer>
    <PropertiesExplorer opened={propertiesExplorerOpen} obj={propertiesExplorerData} wM={wM}></PropertiesExplorer>
  </div>
  <div
    class="pull-out"
    class:out={opened}
    role="button"
    onclick={() => {
      opened = !opened;
    }}
    tabindex="0"
    onkeydown={(ev) => {
      if (ev.key === "Enter") {
        opened = !opened;
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
      class="lucide lucide-chevron-last"
      ><path d="m7 18 6-6-6-6" /><path d="M17 6v12" /></svg
    >
  </div>
</div>

<style>
  .outer-container {
    position: fixed;
    top: 0;
    left: 0;
    transform: translateX(calc(-100% + 38px + 5px)); /*button is 38px in width + 5px padding*/

    display: flex;
    flex-direction: row;
    align-items: flex-start;
    gap: 5px;
    transition: all ease-in-out;
    transition-duration: 500ms;
    width: 400px;
  }
  .outer-container.out {
    transform: translateX(0);
  }
  .pull-out {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 10px;
    margin-top: 14px;
    border-radius: 50%;
    background-color: #2C3036;
    color: white;
    height: 18px;
    width: 18px;
    transform: rotate(0deg);
    transition: all ease-in-out;
    transition-duration: 400ms;
    cursor: pointer;
  }
  .pull-out.out {
    transform: rotate(180deg);
  }
  .drawer {
    height: 100vh;
    box-sizing: border-box;
    padding-top: 8px;
    padding-left: 8px;
    padding-bottom: 8px;
    gap: 12px;
    display: flex;
    flex-direction: column;
  }
</style>

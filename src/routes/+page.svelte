<script lang="ts">
  setContext(commandKeyring, {
    clas: clearLayersAndSources,
    sc: shuffleColours,
    ral: removeALayer,
    scol: setColorOfLayer
  });
  import { onMount, setContext } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import * as maplibregl from "maplibre-gl";
  import { invoke } from "@tauri-apps/api/core";
  import workerUrl from "maplibre-gl/dist/maplibre-gl-worker?worker&url";
  import LeftDrawer from "../widgetsComponents/leftDrawer.svelte";
  import {
    commandKeyring,
    WidgetManager,
    type UserQuestionPayload,
    type Source,
    type Layer,
    type LayerSourceCollection,
  } from "../functional/keys";
  import PopoutQuestion from "../widgetsComponents/popoutQuestion.svelte";
  import "maplibre-gl/dist/maplibre-gl.css";
  maplibregl.setWorkerUrl(workerUrl);

  let map: maplibregl.Map | null = $state(null);
  let widgetManager: WidgetManager = $state(new WidgetManager());

  let uqPayload: UserQuestionPayload = $state({
    question: "",
    options: [],
    q_type: "",
  });
  let showEmittedQuestion = $state(false);

  let addedSources: string[] = [];
  let addedLayers: string[] = $state([]);

  onMount(() => {
    map = new maplibregl.Map({
      container: "map-container",
      style: {
        version: 8,
        sources: {
          "osm-tiles": {
            type: "raster",
            tiles: ["https://tile.openstreetmap.org/{z}/{x}/{y}.png"],
            tileSize: 256,
            attribution:
              '&copy; <a href="https://openstreetmap.org">OpenStreetMap</a> contributors',
          },
        },
        layers: [
          {
            id: "osm-layer",
            type: "raster",
            source: "osm-tiles",
            minzoom: 0,
            maxzoom: 19,
          },
        ],
      },
      center: [0, 0],
      zoom: 0,
    });

    let unlistenGeo: () => void;
    let unlistenInput: () => void;

    (async () => {
      unlistenGeo = await listen("geo-update", (ev) => {
        const lsc = ev.payload as LayerSourceCollection[];
        console.log(lsc);
        clearLayersAndSources();
        lsc.forEach((collection) => {
          deserialiseAndAddToMap(collection.source, collection.layer);
        });
      });

      unlistenInput = await listen("rust-request-input", (ev) => {
        const payload = ev.payload as UserQuestionPayload;
        uqPayload = payload;
        showEmittedQuestion = true;
      });
    })();

    const handleMapLoad = () => {
      // map!.setStyle("https://tiles.openfreemap.org/styles/dark");

      map!.on("click", (ev) => {
        try {
          let feats = map!.queryRenderedFeatures(ev.point)[0];
          if (
            !feats ||
            feats.layer.id == "osm-layer" ||
            feats.source == "openmaptiles"
          ) {
            widgetManager.clearProperties();
            return;
          }
          widgetManager.toggleProperties(feats.properties);
        } catch {
          widgetManager.clearProperties();
        }
      });
    };

    if (map.loaded()) {
      handleMapLoad();
    } else {
      map.on("load", handleMapLoad);
    }

    return () => {
      map?.remove();

      if (unlistenGeo) unlistenGeo();
      if (unlistenInput) unlistenInput();
    };
  });

  function clearLayersAndSources(clear_all?: boolean) {
    addedLayers.forEach((e) => {
      map?.removeLayer(e);
    });
    addedSources.forEach((e) => {
      map?.removeSource(e);
    });

    addedSources = [];
    addedLayers = [];
    if (clear_all == true) {
      widgetManager.clearProperties();
      invoke("clear_all_layers");
    }
  }

  function deserialiseAndAddToMap(source: Source, layer: Layer) {
    let dSource = deserialiseSource(source);
    let dLayer = deserialiseLayer(layer);

    addLSCToMap(source, dSource, layer, dLayer);
  }

  function addLSCToMap(
    source: Source,
    sourceS: maplibregl.SourceSpecification,
    layer: Layer,
    layerS: maplibregl.LayerSpecification,
  ) {
    if (!map) {
      return;
    }

    map.addSource(source.s_id, sourceS);
    map.addLayer(layerS);

    addedSources.push(source.s_id);
    addedLayers.push(layer.l_id);
  }

  function deserialiseSource(source: Source): maplibregl.SourceSpecification {
    switch (source.s_type) {
      case "geojson":
        return {
          type: "geojson",
          data: source.data ? source.data : source.url || "",
        };

      case "vector":
        return {
          type: "vector",
          url: source.url || undefined,
        };

      case "raster":
      case "raster-dem":
        if (source.tiles && source.tiles.length > 0) {
          return {
            type: source.s_type as "raster" | "raster-dem",
            tiles: source.tiles,
            tileSize: source.tile_size ? Number(source.tile_size) : 256,
            minzoom: source.min_zoom ? Number(source.min_zoom) : 0,
            maxzoom: source.max_zoom ? Number(source.max_zoom) : 22,
          };
        } else {
          return {
            type: source.s_type as "raster" | "raster-dem",
            url: source.url || "",
            minzoom: source.min_zoom ? Number(source.min_zoom) : 0,
            maxzoom: source.max_zoom ? Number(source.max_zoom) : 22,
          };
        }

      default:
        return {
          type: "geojson",
          data: source.data || source.url || "",
        };
    }
  }

  function deserialiseLayer(layer: Layer): maplibregl.LayerSpecification {
    const maplibreLayer: any = {
      id: layer.l_id,
      type: layer.l_type,
      source: layer.s_id,
    };

    if (layer.s_layer) {
      console.log("vector");
      maplibreLayer["source-layer"] = layer.s_layer;
    }

    switch (layer.l_type) {
      case "fill":
        maplibreLayer.paint = {
          "fill-color": generateRandomColor(),
          "fill-opacity": 0.6,
          "fill-outline-color": "#1d5bb9",
        };
        break;
      case "line":
        maplibreLayer.paint = {
          "line-color": generateRandomColor(),
          "line-width": 3,
        };
        break;
      case "circle":
        maplibreLayer.paint = {
          "circle-color": generateRandomColor(),
          "circle-radius": 6,
          "circle-stroke-width": 1.5,
          "circle-stroke-color": "#ffffff",
        };
        break;
      case "heatmap":
        maplibreLayer.paint = {
          "heatmap-radius": 20,
          "heatmap-opacity": 0.8,
        };
        break;
      default:
        break;
    }

    return maplibreLayer as maplibregl.LayerSpecification;
  }

  function generateRandomColor() {
    return `#${Math.floor(Math.random() * 16777215)
      .toString(16)
      .padStart(6, "0")}`;
  }

  function removeALayer(layer_name: string) {
    const v = addedLayers.indexOf(layer_name);
    if (v !== -1) {
      addedLayers.splice(v, 1);
    }

    map?.removeLayer(layer_name);
    invoke("delete_a_link", { url: layer_name });
  }

  function shuffleColours() {
    if (!map || addedLayers.length === 0) return;

    addedLayers.forEach((la) => {
      const mLayer = map!.getLayer(la);
      if (!mLayer) return;

      switch (mLayer.type) {
        case "fill":
          map!.setPaintProperty(la, "fill-color", generateRandomColor());
          break;
        case "line":
          map!.setPaintProperty(la, "line-color", generateRandomColor());
          break;
        case "circle":
          map!.setPaintProperty(la, "circle-color", generateRandomColor());
          break;
        default:
          break;
      }
    });
    addedLayers = [...addedLayers];
  }
  function setColorOfLayer(
    layer_id: string,
    layer_type: string,
    color: string,
  ) {
    if (!layer_id||!layer_type||!color) return;

    switch (layer_type) {
      case "fill":
        map!.setPaintProperty(layer_id, "fill-color", color);
        break;
      case "line":
        map!.setPaintProperty(layer_id, "line-color", color);
        break;
      case "circle":
        map!.setPaintProperty(layer_id, "circle-color", color);
        break;
      default:
        break;
    }
    addedLayers = [...addedLayers];
  }
</script>

<div id="map-container"></div>
<div class="left-drawer-style">
  <LeftDrawer bind:wM={widgetManager} bind:map bind:layers={addedLayers}
  ></LeftDrawer>
</div>

<PopoutQuestion bind:questionPopoutOpen={showEmittedQuestion} bind:uqPayload
></PopoutQuestion>

<style>
  .left-drawer-style {
    z-index: 10;
  }
  #map-container {
    position: fixed;
    top: 0px;
    left: 0px;
    width: 100vw;
    height: 100vh;
    z-index: 0;
  }
</style>

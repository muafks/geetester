<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { WidgetManager } from "../functional/keys";
  import { openUrl } from "@tauri-apps/plugin-opener";

  let { link, wM }: { link: string; wM: WidgetManager } = $props();
</script>

<div class="bg">
  <div>
    <h1>Open or Add link?</h1>
    <p>{link}</p>
  </div>

  <div class="options">
    <button
      onclick={() => {
        wM.closeLinkDialog();
      }}
    >
      Cancel
    </button>

    <button
      onclick={() => {
        invoke("add_link", { url: link });
        wM.closeLinkDialog();
      }}
    >
      Add to map
    </button>

    <button
      onclick={() => {
        openUrl(link);
        wM.closeLinkDialog();
      }}
    >
      Open
    </button>
  </div>
</div>

<style>
  .bg {
    display: flex;
    flex-direction: column;
    gap: 5px;
    justify-content: space-between;
    align-items: center;
    position: fixed;
    left: 50vw;
    top: 50vh;
    transform: translate(-50%, -50%);
    z-index: 10;
    background-color: #2c3036;
    color: white;
    border-radius: 12px;
    text-align: center;
    overflow: hidden;
    max-width: 80vw;
  }
  .bg h1,
  .bg p {
    padding-left: 12px;
    padding-right: 12px;
  }
  .options {
    display: flex;
    flex-direction: row;
    width: calc(100% + 2px);
  }
  .options button {
    flex: 1;
    border-radius: 0px;
    background-color: transparent;
    border: white solid 1px;
    color: white;
    transform: translateY(1px);
    cursor: pointer;
  }
</style>

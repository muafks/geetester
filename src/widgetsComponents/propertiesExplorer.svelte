<script lang="ts">
  import type { WidgetManager } from "../functional/keys";

  let { opened, obj, wM }: { opened: boolean; obj?: any; wM: WidgetManager } =
    $props();

  function stringAll(thing: any) {
    if (`${thing}` == "[object Object]") {
      return JSON.stringify(thing).trim();
    } else {
      return `${thing}`.trim();
    }
  }
  function checkAndOpenDialog(link: string) {
    if (!URL.canParse(link.trim())) {
      return;
    }

    wM.openLinkDialog(link);
  }
</script>

<div class="explorer" class:hidden={!opened}>
  <h1>Properties Viewer</h1>
  <table class="viewer-table">
    <thead>
      <tr>
        <th>Key</th>
        <th>Value</th>
      </tr>
    </thead>
    <tbody>
      {#each Object.entries(obj ?? {}) as [ke, val]}
        <tr>
          <td title={ke}>{ke}</td>
          <td
            style={URL.canParse(stringAll(val).trim())
              ? "cursor: pointer;"
              : ""}
            title={`${stringAll(val)}`}
            onclick={() => {
              checkAndOpenDialog(stringAll(val));
            }}>{stringAll(val)}</td
          >
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .explorer {
    display: flex;
    flex-direction: column;
    width: 100%;
    flex: 1 1 0%;
    min-height: 0;
    background-color: #2c3036;
    border-radius: 8px;
    transition: all 250ms ease-in-out;
    max-height: 50%;
    color: white;
    text-align: center;
    max-height: 50%;
    overflow-y: scroll;
    min-width: 0;
    overflow-x: auto;
    overflow-y: auto;
    align-items: center;
  }

  .explorer.hidden {
    flex: 0 0 0px;
    max-height: 0;
    overflow: hidden;
    padding: 0;
  }

  .explorer table {
    width: 90%;
    min-width: 0;
    table-layout: fixed;
    border-collapse: collapse;
    border-style: hidden;
  }

  .explorer table td,
  .explorer table th {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    border: solid white 1px;
    padding-left: 5px;
    padding-right: 5px;
  }
</style>

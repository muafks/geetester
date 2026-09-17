<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, tick } from "svelte";
  import type { UserQuestionPayload } from "../functional/keys";

  let {
    uqPayload = $bindable(),
    questionPopoutOpen = $bindable(),
  }: {
    uqPayload: UserQuestionPayload;
    questionPopoutOpen: boolean;
  } = $props();

  let listPointer = $state(0);
  let inputValue = $state("");

  function moveDir(dir: number) {
    if (uqPayload.options.length === 0) {
      return;
    }

    listPointer += dir;

    if (listPointer >= uqPayload.options.length) {
      listPointer = 0;
    } else if (listPointer < 0) {
      listPointer = uqPayload.options.length - 1;
    }
  }
  const keyDownEvent = (ev: KeyboardEvent) => {
    if (!questionPopoutOpen) {
      return;
    }
    if (ev.key === "ArrowUp") {
      moveDir(-1);
    } else if (ev.key === "ArrowDown") {
      moveDir(1);
    } else if (ev.key === "Enter") {
      selectAndSubmit();
    }
  };
  const selectAndSubmit = async (pos?: number) => {
    if (pos == -1 && inputValue.length > 0) {
      await invoke("submit_web_input", { value: inputValue });
      questionPopoutOpen = false;
      inputValue = "";
      return;
    } else if (pos == -1) {
      return;
    }
    const indexToSubmit = pos !== undefined ? pos : listPointer;

    await invoke("submit_web_input", {
      value: uqPayload.options[indexToSubmit],
    });
    questionPopoutOpen = false;
  };
  function resetPointer() {
    listPointer = 0;
  }
  onMount(() => {
    window.addEventListener("keydown", keyDownEvent);

    return () => {
      window.removeEventListener("keydown", keyDownEvent);
    };
  });
</script>

<div
  class="bg"
  class:open={questionPopoutOpen}
  onoutroend={() => {
    resetPointer();
  }}
>
  <h1>{uqPayload.question}</h1>
  {#if uqPayload.q_type == "choice"}
    <div>
      {#each uqPayload.options as opt}
        <button
          tabindex="-1"
          class:selected={uqPayload.options.indexOf(opt) == listPointer}
          onclick={() => {
            selectAndSubmit(uqPayload.options.indexOf(opt));
          }}
          onmouseenter={() => {
            listPointer = uqPayload.options.indexOf(opt);
          }}>{opt}</button
        >
      {/each}
    </div>
  {:else}
    <div>
      <input
        bind:value={inputValue}
        type="text"
        placeholder="Answer here..."
        onkeypress={(ev) => {
          if (ev.key === "Enter") {
            selectAndSubmit(-1);
          }
        }}
      />
    </div>
  {/if}
</div>

<style>
  .bg {
    display: flex;
    flex-direction: column;
    gap: 5px;
    position: fixed;
    left: 50vw;
    top: calc(0px - 100%);
    transform: translateX(-50%);
    background-color: #2c3036;
    color: white;
    transition: top ease-in-out 1000ms;
    padding-left: 24px;
    padding-right: 24px;
    border-radius: 0px 0px 8px 8px;
    text-align: center;
    max-width: 60vw;
    word-wrap: normal;
  }
  .bg div {
    display: flex;
    flex-direction: column;
  }
  .bg div button {
    font-size: larger;
    color: white;
    background-color: transparent;
    border: transparent;
    padding: 12px 8px;
    cursor: pointer;
    transition: color ease-in-out 100ms;
  }

  .bg div input {
    background-color: transparent;
    width: 600px;
    margin-bottom: 24px;
    border: 1px solid white;
    border-radius: 8px;
    padding: 8px 12px;
    color: white;
    font-size: medium;
  }

  .bg div button:focus {
    outline: none;
  }

  .bg.open div button.selected {
    color: #8c8cff;
  }

  .bg.open {
    position: fixed;
    left: 50vw;
    top: 0px;
    transform: translateX(-50%);
  }
</style>

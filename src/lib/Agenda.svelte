<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'

  let emptyResult = false
  let loading = false
  let orgNodes = []

  onMount(async () => {
    await getOrgAgenda()
  })

  async function getOrgAgenda() {
    await invoke<any[]>('get_all_todos', {}).then((res) => {
      if (res.length) {
        orgNodes = res
      } else {
        emptyResult = true
      }
      loading = false
    })
  }
</script>

<div class="wrapper">
  <button style="display:block; margin-left: auto;" on:click={getOrgAgenda}> Refresh </button>
  {#each orgNodes as section}
    <h2>{section.title}</h2>
    {#each section.nodes as node}
      <div class="todo-wrapper">
        <div class="checkbox" class:checked={node.state === 'DONE'} />
        {node.name}
      </div>
    {/each}
  {/each}
  {#if emptyResult}
    <p>The result was empty!</p>
  {/if}
  {#if loading}
    <p>loading...</p>
  {/if}
</div>

<style>
  .todo-wrapper {
    display: flex;
    align-items: center;
    margin: 8px 0;
    gap: 8px;
  }

  .checkbox {
    width: 25px;
    height: 25px;
    background-color: gray;
  }

  .checkbox.checked {
    background-color: green;
  }
</style>

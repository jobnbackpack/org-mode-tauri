<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'

  let headlines = []
  let emptyResult = false
  let loading = false
  let orgNodes = []

  onMount(async () => {
    await getHeadlines()
    await getOrgAgenda()
  })

  async function getOrgAgenda() {
    await invoke('get_all_todos', {}).then((res) => {
      if (res.length) {
        orgNodes = res
      } else {
        emptyResult = true
      }
      loading = false
    })
  }

  async function getHeadlines() {
    loading = true
    await invoke<string[]>('get_org_children', {}).then((res) => {
      if (res.length) {
        headlines = res
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
      <li>{node}</li>
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
</style>

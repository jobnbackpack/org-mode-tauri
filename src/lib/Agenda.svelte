<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'

  let headlines = []
  let emptyResult = false
  let loading = false

  onMount(async () => {
    await getHeadlines()
    test()
  })

  async function test() {
    await invoke('get_all_todos', {})
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
  <button style="display:block; margin-left: auto;" on:click={getHeadlines}> Refresh </button>
  {#each headlines as headline}
    <h2>{headline}</h2>
    ...
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

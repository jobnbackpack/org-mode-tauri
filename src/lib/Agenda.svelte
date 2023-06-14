<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'
  import type { OrgSection } from './types'
  import AllNodesView from './AllNodesView.svelte'
  import DayView from './DayView.svelte'

  type View = 'AllFiles' | 'Day' | 'Week'

  let emptyResult = false
  let loading = false
  let allOrgFiles = []
  let currentView: View = 'AllFiles'

  onMount(async () => {
    await getAllOrgFiles()
  })

  async function getAllOrgFiles() {
    await invoke<OrgSection[]>('get_all', {}).then((res) => {
      if (res.length) {
        allOrgFiles = res
      } else {
        emptyResult = true
      }
      loading = false
    })
  }
</script>

<div class="wrapper">
  <nav class="nav">
    <button on:click={() => (currentView = 'AllFiles')}> All </button>
    <button on:click={() => (currentView = 'Day')}> Day </button>
    <button class="refresh" on:click={getAllOrgFiles}> Refresh </button>
  </nav>
  {#if allOrgFiles.length}
    {#if currentView === 'AllFiles'}
      <AllNodesView orgFiles={allOrgFiles} />
    {/if}
    {#if currentView === 'Day'}
      <DayView orgFiles={allOrgFiles} />
    {/if}
  {/if}

  {#if emptyResult}
    <p>The result was empty!</p>
  {/if}
  {#if loading}
    <p>loading...</p>
  {/if}
</div>

<style>
  .nav {
    display: flex;
    gap: 6px;
  }

  .refresh {
    margin-left: auto;
  }
</style>

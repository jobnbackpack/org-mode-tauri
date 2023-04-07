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
    await invoke<OrgSection[]>('get_all_org_files', {}).then((res) => {
      if (res.length) {
        allOrgFiles = res
        console.log(allOrgFiles)
      } else {
        emptyResult = true
      }
      loading = false
    })
  }
</script>

<div class="wrapper">
  <button style="display:block;" on:click={() => (currentView = 'AllFiles')}> All </button>
  <button style="display:block;" on:click={() => (currentView = 'Day')}> Day </button>
  <button style="display:block; margin-left: auto;" on:click={getAllOrgFiles}> Refresh </button>
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
</style>

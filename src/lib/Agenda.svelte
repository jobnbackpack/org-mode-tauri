<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'
  import type { OrgSection } from './types'
  import SingleFile from './SingleFile.svelte'
  import AllNodesView from './AllNodesView.svelte'

  let emptyResult = false
  let loading = false
  let orgNodes = []
  let allOrgFiles = []

  onMount(async () => {
    await getOrgChildren()
    await getAllOrgChildren()
  })

  async function getOrgChildren() {
    await invoke<OrgSection[]>('get_org_file', {}).then((res) => {
      if (res.length) {
        orgNodes = res
        console.log(orgNodes)
      } else {
        emptyResult = true
      }
      loading = false
    })
  }

  async function getAllOrgChildren() {
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
  <button style="display:block; margin-left: auto;" on:click={getOrgChildren}> Refresh </button>
  {#if orgNodes.length}
    <!-- <SingleFile {orgNodes} /> -->
    <AllNodesView orgFiles={allOrgFiles} />
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

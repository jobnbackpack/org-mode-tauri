<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'

  let emptyResult = false
  let loading = false
  let orgNodes = []

  onMount(async () => {
    await getOrgFile()
  })

  async function getOrgFile() {
    await invoke<any[]>('get_org_file', {}).then((res) => {
      if (res.length) {
        orgNodes = res
        console.log(orgNodes)
      } else {
        emptyResult = true
      }
      loading = false
    })
  }

  function mapDate(timestamp) {
    let date = new Date()
    date.setFullYear(timestamp.year)
    date.setMonth(timestamp.month - 1)
    date.setDate(timestamp.day)
    return date
  }
</script>

<div class="wrapper">
  <button style="display:block; margin-left: auto;" on:click={getOrgFile}> Refresh </button>
  {#each orgNodes as section}
    <h2>{section.title}</h2>
    {#each section.nodes as node}
      <div class="todo-wrapper">
        <div class="checkbox" class:checked={node.state === 'DONE'} />
        <section>
          {node.name}
          <div>
            {#if node.planning?.deadline}
              <span class="date">
                Deadline: {mapDate(node.planning.deadline.start).toDateString()}
              </span>
            {/if}
            {#if node.planning?.scheduled}
              <span class="date">
                Scheduled: {mapDate(node.planning.scheduled.start).toDateString()}
              </span>
            {/if}
            {#if node.planning?.closed}
              <span class="date">
                Closed: {mapDate(node.planning.closed.start).toDateString()}
              </span>
            {/if}
          </div>
        </section>
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

  .date {
    font-style: italic;
  }
</style>

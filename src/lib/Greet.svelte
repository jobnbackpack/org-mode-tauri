<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'

  let name = ''
  let greetMsg = ''
  let headlines = []
  let emptyResult = false
  let loading = false

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke('greet', { name })
  }

  async function getRawOrgJson() {
    loading = true
    let parsedJson: any
    await invoke<string>('get_org_file_json', {}).then((res) => {
      if (res) {
        parsedJson = JSON.parse(res)
      } else {
        emptyResult = true
      }
      loading = false
      console.log('Result: ', parsedJson)
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

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}> Greet </button>
  </div>
  <p>{greetMsg}</p>
  <button on:click={getHeadlines}> Parse! </button>
  {#each headlines as headline}
    <li>{headline}</li>
  {/each}
  {#if emptyResult}
    <p>The result was empty!</p>
  {/if}
  {#if loading}
    <p>loading...</p>
  {/if}
</div>

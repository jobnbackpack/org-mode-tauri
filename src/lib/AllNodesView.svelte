<script lang="ts">
  import Node from './Node.svelte'
  import type { OrgFile } from './types'

  export let orgFiles: OrgFile[] = []
</script>

{#if orgFiles.length}
  {#each orgFiles as file}
    {#if file.name !== 'archive.org'}
      <h1>{file.name}</h1>
      {#each file.nodes as node}
        <Node {node} />
      {/each}
    {/if}
  {/each}
{/if}

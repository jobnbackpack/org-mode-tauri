<script lang="ts">
  import Node from './Node.svelte'
  import type { OrgNode } from './types'

  type OrgFiles = OrgNode[][]
  export let orgFiles: OrgFiles = []
</script>

{#if orgFiles.length}
  {#each orgFiles as fileNodes}
    ---
    {#each fileNodes as node}
      <Node {node} />
    {/each}
  {/each}
{/if}

<script lang="ts">
  import type { OrgNode } from './types'
  import { getDeadlineRange, mapDate } from './utils'

  export let node: OrgNode
  export let withIndent: boolean = true

  function get_dl_days(deadline: Date) {
    return getDeadlineRange(deadline)
  }
</script>

{#if node.state === 'DONE' || node.state === 'TODO'}
  <div class={`todo-wrapper ${withIndent ? 'indent' : ''} level-${node.level}`}>
    <div class="checkbox" class:checked={node.state === 'DONE'} />
    <section>
      {node.name}
      <div>
        {#if node.planning?.deadline}
          <span class="date">
            Deadline: {mapDate(node.planning.deadline).toDateString()} - in {get_dl_days(
              mapDate(node.planning.deadline),
            )} Days
          </span>
        {/if}
        {#if node.planning?.scheduled}
          <span class="date">
            Scheduled: {mapDate(node.planning.scheduled).toDateString()}
          </span>
        {/if}
        {#if node.planning?.closed}
          <span class="date">
            Closed: {mapDate(node.planning.closed).toDateString()}
          </span>
        {/if}
      </div>
    </section>
  </div>
  {#if node.nodes && node.nodes.length}
    {#each node.nodes as sub_node}
      <svelte:self node={sub_node} {withIndent} />
    {/each}
  {/if}
{/if}
{#if node.state === 'NONE'}
  <h2>{node.name}</h2>
  {#if node.nodes && node.nodes.length}
    {#each node.nodes as sub_node}
      <svelte:self node={sub_node} />
    {/each}
  {/if}
{/if}

<style>
  .indent.level-1 {
    margin-inline-start: 0px;
  }
  .indent.level-2 {
    margin-inline-start: 25px;
  }
  .indent.level-3 {
    margin-inline-start: 50px;
  }
</style>

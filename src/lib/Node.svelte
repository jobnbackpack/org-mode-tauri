<script lang="ts">
  import type { OrgNode } from './types'
  import { getDeadlineRange, mapDate } from './utils'

  export let node: OrgNode
  export let withIndent: boolean = true
  const date_locale = 'de'

  function get_dl_days(deadline: Date) {
    return getDeadlineRange(deadline)
  }
</script>

{#if node.state === 'DONE' || node.state === 'TODO'}
  <div class={`todo-wrapper ${withIndent ? 'indent' : ''} level-${node.level}`}>
    <div class="checkbox {node.priority ? node.priority : ''}" class:checked={node.state === 'DONE'} />
    <section>
      <h3>{node.name}</h3>
      <div>
        {#if node.planning?.deadline}
          <span class="date">
            Deadline: {mapDate(node.planning.deadline).toLocaleDateString(date_locale)} -
            <span class="deadline_counter {get_dl_days(mapDate(node.planning.deadline)) < 5 ? 'near_deadline' : ''}">
              in {get_dl_days(mapDate(node.planning.deadline))} Days</span>
          </span>
        {/if}
        {#if node.planning?.scheduled}
          <span class="date">
            Scheduled: {mapDate(node.planning.scheduled).toLocaleDateString(date_locale)}
          </span>
        {/if}
        {#if node.planning?.closed}
          <span class="date">
            Closed: {mapDate(node.planning.closed).toLocaleDateString(date_locale)}
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
  .deadline_counter {
    color: #df7857;
  }
  .near_deadline {
    color: #ea5455;
  }
  .indent.level-1 {
    margin-inline-start: 0px;
  }
  .indent.level-2 {
    margin-inline-start: 25px;
  }
  .indent.level-3 {
    margin-inline-start: 50px;
  }

  .checkbox.A {
    border: solid #ea5455 3px;
  }
  .checkbox.B {
    border: solid #df7857 3px;
  }
  .checkbox.C {
    border: solid #ffebb4 3px;
  }
</style>

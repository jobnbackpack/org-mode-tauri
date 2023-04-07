<script>
  import { mapDate } from './utils'

  export let orgNodes
</script>

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
              Deadline: {mapDate(node.planning.deadline).toDateString()}
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
  {/each}
{/each}

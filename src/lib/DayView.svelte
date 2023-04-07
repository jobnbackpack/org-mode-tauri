<script lang="ts">
  import { onMount } from 'svelte'
  import Node from './Node.svelte'
  import type { OrgFile, OrgNode } from './types'
  import { mapDate } from './utils'

  export let orgFiles: OrgFile[] = []
  let todos: OrgNode[] = []

  onMount(async () => {
    let allTodos = await filterAllTodos()
    let todosWithDate = allTodos.filter((todo) => todo.planning)
    todos = todosWithDate.filter((todo) => {
      let result = false
      if (todo.planning.scheduled) {
        result = isToday(mapDate(todo.planning.scheduled))
      }
      if (todo.planning.deadline) {
        result = isToday(mapDate(todo.planning.deadline))
      }
      return result
    })
  })

  function isToday(date: Date): boolean {
    const today = new Date()
    return (
      date.getFullYear() === today.getFullYear() &&
      date.getMonth() === today.getMonth() &&
      date.getDate() === today.getDate()
    )
  }

  async function filterAllTodos(): Promise<OrgNode[]> {
    let excludedFiles = ['archive.org']
    let filterStates = ['DONE', 'TODO']
    let result = []

    for (let file of orgFiles) {
      if (!excludedFiles.includes(file.name)) {
        let toplevel_todos = file.nodes.filter((node) => filterStates.includes(node.state))
        let sublevel_todos = []
        let subsublevel_todos = []
        for (let todo of toplevel_todos) {
          sublevel_todos = [...sublevel_todos, ...todo.nodes.filter((t) => filterStates.includes(t.state))]
          for (let task of todo.nodes) {
            subsublevel_todos = [...subsublevel_todos, ...task.nodes.filter((a) => filterStates.includes(a.state))]
            //TODO: possible headlines
          }
        }
        let headings = file.nodes.filter((node) => !filterStates.includes(node.state))
        for (let heading of headings) {
          sublevel_todos = [...sublevel_todos, ...heading.nodes.filter((t) => filterStates.includes(t.state))]
          for (let task of heading.nodes) {
            subsublevel_todos = [...subsublevel_todos, ...task.nodes.filter((a) => filterStates.includes(a.state))]
            //TODO: possible headlines
          }
        }
        result = [...result, ...toplevel_todos]
        result = [...result, ...sublevel_todos]
        result = [...result, ...subsublevel_todos]
      }
    }
    return result
  }
</script>

{#if todos.length}
  {#each todos as node}
    <Node withIndent={false} {node} />
  {/each}
{/if}

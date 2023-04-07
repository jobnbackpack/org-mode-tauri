export interface OrgSection {
  nodes: OrgNode[]
  title: string
}

export interface OrgNode {
  level: number
  name: string
  planning?: OrgPlanning
  state: OrgTodoState
}

export type OrgTodoState = 'TODO' | 'DONE' | null

export interface OrgPlanning {
  deadline?: OrgTimestamp
  scheduled?: OrgTimestamp
  closed?: OrgTimestamp
}

export interface OrgTimestamp {
  start: {
    year: number
    month: number
    day: number
    dayName: string
    hour?: number
    minute?: number
  }
  timestamp_type: 'inactive'
}

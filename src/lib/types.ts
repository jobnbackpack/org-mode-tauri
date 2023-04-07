export interface OrgSection {
  nodes: OrgNode[]
  title: string
}

export interface OrgNode {
  name: string
  level: number
  state: OrgTodoState
  planning?: OrgPlanning
  nodes?: OrgNode[]
}

export type OrgTodoState = 'TODO' | 'DONE' | 'NONE'

export interface OrgFile {
  name: string
  path: string
  nodes: OrgNode[]
}

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

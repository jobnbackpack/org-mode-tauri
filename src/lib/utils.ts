import type { OrgTimestamp } from './types'

export function mapDate(timestamp: OrgTimestamp) {
  let date = new Date()
  date.setFullYear(timestamp.start.year)
  date.setMonth(timestamp.start.month - 1)
  date.setDate(timestamp.start.day)
  return date
}

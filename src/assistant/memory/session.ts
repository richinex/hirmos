// The active chat session id: the scope for turns, DAG nodes, and the REPL namespace. Rotating it
// IS "new chat": the old session's DAG stays in IndexedDB under its id; the new one starts blank.
const KEY = 'hirmos_assistant_session'

export function getActiveSessionId(project: string): string {
  const key = `${KEY}:${project}`
  const id = localStorage.getItem(key)
  return id?.startsWith(`${project}:`) ? id : rotateSession(project)
}

export function rotateSession(project: string): string {
  const id = `${project}:${crypto.randomUUID()}`
  localStorage.setItem(`${KEY}:${project}`, id)
  return id
}

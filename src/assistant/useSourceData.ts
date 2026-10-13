import { useLayoutEffect } from 'react'
import type { Workflow } from '@/domain/workflow'
import { useAssistantBridge } from './applicationTools'

export function useSourceData(workflow: Workflow) {
  const bridge = useAssistantBridge()
  const source = 'source' in workflow ? workflow.source : null
  const project = 'project' in workflow ? workflow.project.id : null
  const fingerprint = workflow.kind === 'profiled' ? workflow.profile.source.fingerprint : null
  useLayoutEffect(() => {
    if (!source || !project) return
    bridge.scopeProfiles(project)
    return bridge.publishSourceData({
      kind: 'source',
      revision: crypto.randomUUID(),
      inspection: {
        kind: 'source',
        name: source.name,
        tables: ['source'],
        meaning:
          'The accepted source, before preprocessing. This is not the transformed or filtered estimation sample.',
      },
      profile: async (request, signal) => {
        const { profileAcceptedSource } = await import('./sourceProfile')
        return profileAcceptedSource(source, fingerprint, request, signal)
      },
    })
  }, [bridge, source, project, fingerprint])
}

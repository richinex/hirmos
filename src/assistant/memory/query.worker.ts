/// <reference lib="webworker" />
import { dagHistory, dagGrep, dagExpand, dagDescribe } from './dagStore'

self.onmessage = async ({
  data,
}: MessageEvent<{ session: string; name: string; args: Record<string, unknown> }>) => {
  try {
    const { session, name, args } = data
    const result =
      name === 'DAG_HISTORY'
        ? await dagHistory(
            session,
            args.kind as string | undefined,
            args.limit as number | undefined,
            args.turn_id as string | undefined,
          )
        : name === 'DAG_GREP'
          ? await dagGrep(session, String(args.pattern))
          : name === 'DAG_EXPAND'
            ? await dagExpand(session, String(args.node_id))
            : name === 'DAG_DESCRIBE'
              ? await dagDescribe(session)
              : null
    self.postMessage({ ok: true, result })
  } catch {
    self.postMessage({ ok: false, error: 'Conversation memory could not be read.' })
  }
}

export function openRouterStream(reply: {
  choices: readonly { message: { content: string | null }; finish_reason?: string }[]
}) {
  const choice = reply.choices[0]
  const chunk = (delta: unknown, finish: string | null = null) => ({
    id: 'test-completion',
    object: 'chat.completion.chunk',
    created: 1,
    model: 'test/model',
    choices: [{ index: 0, delta, finish_reason: finish }],
  })
  return {
    contentType: 'text/event-stream',
    body:
      [
        chunk({ role: 'assistant', content: choice.message.content ?? '' }),
        chunk({}, choice.finish_reason ?? 'stop'),
      ]
        .map((item) => `data: ${JSON.stringify(item)}\n\n`)
        .join('') + 'data: [DONE]\n\n',
  }
}

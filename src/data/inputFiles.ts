import type { SqlInputDescriptor, SqlPreparationInput } from '@/domain/sourceInputs'

/**
 * The File handles behind the inputs a session has read, kept for the life of the page and never
 * persisted: a recipe stores only descriptors, so reopening an editor in the same session can take
 * its files from here instead of asking for them again. After a reload the map is empty and the
 * files are asked for.
 */
// One map for the page, whatever module instance asks: a hot reload of this file must not lose the handles.
const registry = globalThis as { hirmosInputFiles?: Map<string, File> }
const byFingerprint = registry.hirmosInputFiles ??= new Map<string, File>()

export const rememberInputFiles = (inputs: readonly SqlPreparationInput[]): void => {
  for (const input of inputs) byFingerprint.set(input.fingerprint, input.file)
}

/** The inputs a recipe names, from the remembered handles, or null when any is missing. */
export const inputsFromMemory = (descriptors: readonly SqlInputDescriptor[]): readonly SqlPreparationInput[] | null => {
  const inputs: SqlPreparationInput[] = []
  for (const descriptor of descriptors) {
    const file = byFingerprint.get(descriptor.fingerprint)
    if (file === undefined) return null
    inputs.push({ ...descriptor, file })
  }
  return inputs
}

/** Drops every remembered handle, as a reload would. */
export const forgetInputFiles = (): void => { byFingerprint.clear() }

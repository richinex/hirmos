interface GpuAccess {
  requestAdapter(): Promise<unknown | null>
}

export async function embeddingBackend(
  gpu: GpuAccess | undefined,
): Promise<
  | { readonly device: 'webgpu'; readonly dtype: 'fp32' }
  | { readonly device: 'wasm'; readonly dtype: 'q8' }
> {
  if (gpu) {
    try {
      if (await gpu.requestAdapter()) return { device: 'webgpu', dtype: 'fp32' }
    } catch {
      // Some browsers expose WebGPU but deny adapter access.
    }
  }
  return { device: 'wasm', dtype: 'q8' }
}

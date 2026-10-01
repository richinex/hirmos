import { memo, useMemo } from 'react'
import rough from 'roughjs'

/** Stable geometry: selection and theme change colour, never the random stroke. */
export default memo(function SketchStroke({ id, path, style }: {
  readonly id: string
  readonly path: string
  readonly style?: React.CSSProperties
}) {
  const paths = useMemo(() => {
    let seed = 17
    for (const character of id) seed = ((seed * 31 + character.charCodeAt(0)) >>> 0)
    const generator = rough.generator()
    return generator.toPaths(generator.path(path, {
      seed: seed || 1, roughness: 0.55, bowing: 0.3, maxRandomnessOffset: 0.7,
      preserveVertices: true, disableMultiStroke: true,
    }))
  }, [id, path])
  return <g data-sketch-stroke={id} style={{ ...style, pointerEvents: 'none' }}>
    {paths.map((stroke, index) => <path key={index} d={stroke.d} fill="none" stroke="currentColor" />)}
  </g>
})

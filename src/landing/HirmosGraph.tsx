import { useEffect, useRef } from 'react'
import * as THREE from 'three'
import { Line2 } from 'three/examples/jsm/lines/Line2.js'
import { LineGeometry } from 'three/examples/jsm/lines/LineGeometry.js'
import { LineMaterial } from 'three/examples/jsm/lines/LineMaterial.js'
import { EffectComposer } from 'three/examples/jsm/postprocessing/EffectComposer.js'
import { OutputPass } from 'three/examples/jsm/postprocessing/OutputPass.js'
import { RenderPass } from 'three/examples/jsm/postprocessing/RenderPass.js'
import { UnrealBloomPass } from 'three/examples/jsm/postprocessing/UnrealBloomPass.js'

/**
 * The hero scene tells the headline: raw data becomes a causal estimate. A cloud of observations
 * drifts, condenses into the four variables, the arrows of the stated DAG draw on, and the effect
 * pulses along the causal path. Colour comes from the theme tokens; under reduced motion the scene
 * renders its final frame once.
 */

type Point = readonly [x: number, y: number, z: number]

interface GraphNode {
  readonly id: 'C' | 'X' | 'M' | 'Y'
  readonly caption: string
  readonly point: Point
  readonly share: number
}

interface GraphEdge {
  readonly from: GraphNode['id']
  readonly to: GraphNode['id']
  readonly emphasis: 'structure' | 'effect'
  readonly bend: number
}

const NODES: readonly GraphNode[] = [
  { id: 'C', caption: 'common cause', point: [-0.35, 1.24, 0], share: 0.2 },
  { id: 'X', caption: 'treatment', point: [-1.45, 0.02, 0], share: 0.3 },
  { id: 'M', caption: 'mediator', point: [0.05, 0.22, 0], share: 0.18 },
  { id: 'Y', caption: 'outcome', point: [1.45, -0.56, 0], share: 0.32 },
] as const

const EDGES: readonly GraphEdge[] = [
  { from: 'C', to: 'X', emphasis: 'structure', bend: 0.1 },
  { from: 'C', to: 'Y', emphasis: 'structure', bend: -0.14 },
  { from: 'X', to: 'M', emphasis: 'effect', bend: 0.1 },
  { from: 'M', to: 'Y', emphasis: 'effect', bend: -0.08 },
  { from: 'X', to: 'Y', emphasis: 'effect', bend: -0.34 },
] as const

const NODE_RADIUS = 0.225
const SEGMENTS = 48

const point = ([x, y, z]: Point): THREE.Vector3 => new THREE.Vector3(x, y, z)

const colourToken = (style: CSSStyleDeclaration, token: string, fallback: string): THREE.Color => {
  const value = style.getPropertyValue(token).trim() || fallback
  const canvas = document.createElement('canvas')
  canvas.width = 1
  canvas.height = 1
  const context = canvas.getContext('2d', { willReadFrequently: true })
  if (context === null) return new THREE.Color(fallback)

  context.fillStyle = fallback
  context.fillStyle = value
  context.fillRect(0, 0, 1, 1)
  const [red, green, blue] = context.getImageData(0, 0, 1, 1).data
  const hex = [red, green, blue].map((channel) => channel.toString(16).padStart(2, '0')).join('')
  return new THREE.Color(`#${hex}`)
}

/** A deterministic generator, so the cloud is the same on every visit and the entrance can be tuned. */
const random = (() => {
  let seed = 0x9e3779b9
  return () => {
    seed = (Math.imul(seed ^ (seed >>> 15), seed | 1) + 0x6d2b79f5) >>> 0
    return ((seed ^ (seed >>> 13)) >>> 0) / 4294967296
  }
})()

function textSprite(text: string, colour: THREE.Color, width: number, fontSize: number): THREE.Sprite {
  const canvas = document.createElement('canvas')
  canvas.width = 512
  canvas.height = 160
  const context = canvas.getContext('2d')
  if (context !== null) {
    context.fillStyle = `#${colour.getHexString()}`
    context.font = `500 ${fontSize}px "JetBrains Mono Variable", ui-monospace, monospace`
    context.textAlign = 'center'
    context.textBaseline = 'middle'
    context.fillText(text, canvas.width / 2, canvas.height / 2)
  }
  const texture = new THREE.CanvasTexture(canvas)
  texture.colorSpace = THREE.SRGBColorSpace
  texture.minFilter = THREE.LinearFilter
  const sprite = new THREE.Sprite(new THREE.SpriteMaterial({ map: texture, transparent: true, depthWrite: false }))
  sprite.scale.set(width, width * (canvas.height / canvas.width), 1)
  return sprite
}

/** A soft disc for the observation points: a radial falloff drawn once and shared. */
function discTexture(): THREE.Texture {
  const canvas = document.createElement('canvas')
  canvas.width = 64
  canvas.height = 64
  const context = canvas.getContext('2d')
  if (context !== null) {
    const gradient = context.createRadialGradient(32, 32, 0, 32, 32, 32)
    gradient.addColorStop(0, 'rgba(255,255,255,1)')
    gradient.addColorStop(0.45, 'rgba(255,255,255,0.75)')
    gradient.addColorStop(1, 'rgba(255,255,255,0)')
    context.fillStyle = gradient
    context.fillRect(0, 0, 64, 64)
  }
  const texture = new THREE.CanvasTexture(canvas)
  texture.colorSpace = THREE.SRGBColorSpace
  return texture
}

function edgeCurve(from: THREE.Vector3, to: THREE.Vector3, bend: number): THREE.QuadraticBezierCurve3 {
  const direction = to.clone().sub(from).normalize()
  const start = from.clone().addScaledVector(direction, NODE_RADIUS + 0.06)
  const end = to.clone().addScaledVector(direction, -(NODE_RADIUS + 0.2))
  const control = start.clone().lerp(end, 0.5)
  control.y += bend
  return new THREE.QuadraticBezierCurve3(start, control, end)
}

function arrowHead(curve: THREE.QuadraticBezierCurve3, colour: THREE.Color, size: number): THREE.Mesh {
  const mesh = new THREE.Mesh(new THREE.ConeGeometry(size * 0.42, size, 18), new THREE.MeshBasicMaterial({ color: colour }))
  const endpoint = curve.getPoint(1)
  const tangent = curve.getTangent(1).normalize()
  mesh.position.copy(endpoint).addScaledVector(tangent, size * 0.45)
  mesh.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), tangent)
  return mesh
}

const easeOut = (t: number) => 1 - (1 - t) ** 3
const easeInOut = (t: number) => (t < 0.5 ? 4 * t ** 3 : 1 - (-2 * t + 2) ** 3 / 2)
const clamp01 = (t: number) => Math.min(1, Math.max(0, t))

export function HirmosGraph({ className }: { readonly className?: string }) {
  const host = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const element = host.current
    if (element === null) return

    const style = getComputedStyle(document.documentElement)
    const stage = colourToken(style, '--color-stage', '#0A0A0B')
    const ink = colourToken(style, '--color-ink', '#E6E3DC')
    const bone = colourToken(style, '--color-bone', '#B8B5AE')
    const muted = colourToken(style, '--color-muted', '#8C8A85')
    const signal = colourToken(style, '--color-signal', '#E4501F')
    const dark = stage.getHSL({ h: 0, s: 0, l: 0 }).l < 0.5
    const still = window.matchMedia('(prefers-reduced-motion: reduce)').matches
    const compact = window.matchMedia('(max-width: 900px)').matches

    const scene = new THREE.Scene()
    scene.fog = new THREE.FogExp2(stage.getHex(), dark ? 0.055 : 0.042)
    const camera = new THREE.PerspectiveCamera(36, 1, 0.1, 40)
    camera.position.set(0, 0, 9)

    let renderer: THREE.WebGLRenderer
    try {
      renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true, powerPreference: 'low-power' })
    } catch {
      return
    }
    renderer.setClearColor(stage, 0)
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, compact ? 1.5 : 2))
    renderer.outputColorSpace = THREE.SRGBColorSpace
    renderer.toneMapping = THREE.ACESFilmicToneMapping
    renderer.toneMappingExposure = 1.12
    element.prepend(renderer.domElement)

    // Bloom lifts the signal arrows and live rings off the ground. It costs a second render target, so
    // it is spent only on a dark ground with room to draw, and never when motion is refused.
    const composer = !compact && !still && dark ? new EffectComposer(renderer) : null
    if (composer !== null) {
      composer.addPass(new RenderPass(scene, camera))
      composer.addPass(new UnrealBloomPass(new THREE.Vector2(1, 1), 0.42, 0.5, 0.72))
      composer.addPass(new OutputPass())
    }

    const graph = new THREE.Group()
    scene.add(graph)

    // The observations: each point starts scattered and belongs to one variable, where it settles
    // into a loose orbit around the node. Treatment and outcome carry the signal tint.
    const nodePositions = new Map<GraphNode['id'], THREE.Vector3>(NODES.map((node) => [node.id, point(node.point)]))
    const count = compact ? 420 : 760
    const scatter = new Float32Array(count * 3)
    const home = new Float32Array(count * 3)
    const phase = new Float32Array(count)
    const delay = new Float32Array(count)
    const positions = new Float32Array(count * 3)
    const colours = new Float32Array(count * 3)
    const owners: GraphNode[] = []
    {
      let cursor = 0
      NODES.forEach((node, index) => {
        const quota = index === NODES.length - 1 ? count - cursor : Math.round(count * node.share)
        for (let n = 0; n < quota && cursor < count; n += 1, cursor += 1) owners.push(node)
      })
    }
    for (let index = 0; index < count; index += 1) {
      const owner = owners[index] ?? NODES[0]
      const centre = nodePositions.get(owner.id) ?? new THREE.Vector3()
      scatter[index * 3] = (random() - 0.5) * 6.4
      scatter[index * 3 + 1] = (random() - 0.5) * 3.8 + 0.3
      scatter[index * 3 + 2] = (random() - 0.6) * 2.2
      const angle = random() * Math.PI * 2
      const radius = NODE_RADIUS + 0.12 + random() ** 1.6 * 0.5
      home[index * 3] = centre.x + Math.cos(angle) * radius
      home[index * 3 + 1] = centre.y + Math.sin(angle) * radius * 0.82
      home[index * 3 + 2] = (random() - 0.5) * 0.3
      phase[index] = random() * Math.PI * 2
      delay[index] = 0.15 + random() * 1.1
      const tint = (owner.id === 'X' || owner.id === 'Y' ? signal : bone).clone().lerp(stage, dark ? 0.15 : 0.05)
      colours[index * 3] = tint.r
      colours[index * 3 + 1] = tint.g
      colours[index * 3 + 2] = tint.b
    }
    const cloudGeometry = new THREE.BufferGeometry()
    const positionAttribute = new THREE.BufferAttribute(positions, 3)
    positionAttribute.setUsage(THREE.DynamicDrawUsage)
    cloudGeometry.setAttribute('position', positionAttribute)
    cloudGeometry.setAttribute('color', new THREE.BufferAttribute(colours, 3))
    const cloud = new THREE.Points(cloudGeometry, new THREE.PointsMaterial({
      map: discTexture(),
      vertexColors: true,
      size: compact ? 0.06 : 0.07,
      sizeAttenuation: true,
      transparent: true,
      opacity: dark ? 0.55 : 0.62,
      depthWrite: false,
    }))
    graph.add(cloud)

    // The variables: flat discs so the glyph sits in front, a ring in the node's tone, the caption below.
    const nodeGroups: THREE.Group[] = []
    for (const node of NODES) {
      const group = new THREE.Group()
      group.position.copy(nodePositions.get(node.id) ?? new THREE.Vector3())
      const live = node.id === 'X' || node.id === 'Y'
      const disc = new THREE.Mesh(new THREE.CircleGeometry(NODE_RADIUS, 64), new THREE.MeshBasicMaterial({ color: stage.clone().lerp(ink, dark ? 0.06 : 0.02) }))
      const ring = new THREE.Mesh(new THREE.RingGeometry(NODE_RADIUS, NODE_RADIUS + 0.018, 96), new THREE.MeshBasicMaterial({ color: live ? signal : bone, transparent: true, opacity: live ? 0.95 : 0.8 }))
      ring.position.z = 0.002
      const glyph = textSprite(node.id, live ? signal : ink, 0.4, 118)
      glyph.position.z = 0.03
      const caption = textSprite(node.caption, muted, 1.15, 26)
      caption.position.set(0, -(NODE_RADIUS + 0.22), 0.03)
      group.add(disc, ring, glyph, caption)
      group.scale.setScalar(0)
      group.userData = { ring, live, pulseSpeed: 0.42 + Math.random() * 0.34, pulseOffset: Math.random() * Math.PI * 2 }
      graph.add(group)
      nodeGroups.push(group)
    }

    // The arrows: fat lines with a constant pixel width, drawn on segment by segment.
    const lineResolution = new THREE.Vector2(1, 1)
    const lineMaterials: LineMaterial[] = []
    const edges: Array<{ readonly line: Line2; readonly head: THREE.Mesh; readonly curve: THREE.QuadraticBezierCurve3; readonly emphasis: GraphEdge['emphasis'] }> = []
    for (const edge of EDGES) {
      const from = nodePositions.get(edge.from)
      const to = nodePositions.get(edge.to)
      if (from === undefined || to === undefined) continue
      const curve = edgeCurve(from, to, edge.bend)
      const geometry = new LineGeometry()
      geometry.setPositions(curve.getPoints(SEGMENTS).flatMap((p) => [p.x, p.y, p.z]))
      const effect = edge.emphasis === 'effect'
      const material = new LineMaterial({
        color: (effect ? signal : bone).getHex(),
        linewidth: effect ? 2.6 : 1.8,
        transparent: true,
        opacity: effect ? 0.95 : dark ? 0.6 : 0.8,
        resolution: lineResolution,
        worldUnits: false,
      })
      lineMaterials.push(material)
      const line = new Line2(geometry, material)
      line.computeLineDistances()
      const head = arrowHead(curve, effect ? signal : bone, effect ? 0.2 : 0.16)
      head.visible = false
      graph.add(line, head)
      edges.push({ line, head, curve, emphasis: edge.emphasis })
    }
    const pulses = edges.filter((edge) => edge.emphasis === 'effect').map((edge, index) => {
      const mesh = new THREE.Mesh(new THREE.SphereGeometry(0.045, 16, 10), new THREE.MeshBasicMaterial({ color: signal }))
      mesh.visible = false
      graph.add(mesh)
      return { mesh, curve: edge.curve, offset: index * 0.31 }
    })

    // Entrance clock: cloud condenses, nodes spring in as their points arrive, arrows draw on, pulses follow.
    const CONDENSE = 1.5
    const NODE_AT = 1.25
    const NODE_STEP = 0.16
    const EDGE_AT = NODE_AT + NODE_STEP * NODES.length + 0.3
    const EDGE_STEP = 0.16
    const EDGE_DURATION = 0.6
    const SETTLED = EDGE_AT + EDGE_STEP * edges.length + EDGE_DURATION
    const springs = nodeGroups.map(() => ({ scale: 0, velocity: 0 }))

    const layoutCloud = (time: number) => {
      const attribute = cloudGeometry.getAttribute('position') as THREE.BufferAttribute
      for (let index = 0; index < count; index += 1) {
        const t = still ? 1 : easeInOut(clamp01((time - delay[index]!) / CONDENSE))
        const wobble = Math.sin(time * 0.7 + phase[index]!) * 0.035
        const drift = still ? 0 : (1 - t) * Math.sin(time * 0.35 + phase[index]!) * 0.12
        const sx = scatter[index * 3]! + drift
        const sy = scatter[index * 3 + 1]! + Math.cos(time * 0.3 + phase[index]!) * 0.08 * (1 - t)
        const sz = scatter[index * 3 + 2]!
        attribute.setXYZ(
          index,
          sx + (home[index * 3]! + wobble - sx) * t,
          sy + (home[index * 3 + 1]! + wobble * 0.6 - sy) * t,
          sz + (home[index * 3 + 2]! - sz) * t,
        )
      }
      attribute.needsUpdate = true
    }

    const layoutGraph = (time: number) => {
      springs.forEach((spring, index) => {
        if (still) { nodeGroups[index]?.scale.setScalar(1); return }
        if (time < NODE_AT + index * NODE_STEP) return
        const force = -0.16 * (spring.scale - 1)
        spring.velocity = (spring.velocity + force) * 0.84
        spring.scale += spring.velocity
        nodeGroups[index]?.scale.setScalar(Math.max(0, spring.scale))
      })
      edges.forEach(({ line, head }, index) => {
        const progress = still ? 1 : clamp01((time - EDGE_AT - index * EDGE_STEP) / EDGE_DURATION)
        line.geometry.instanceCount = Math.round(SEGMENTS * easeOut(progress))
        head.visible = progress >= 1
      })
      const settled = still || time >= SETTLED
      pulses.forEach(({ mesh, curve, offset }) => {
        mesh.visible = settled
        if (!settled) return
        const progress = (time * 0.16 + offset) % 1
        mesh.position.copy(curve.getPoint(progress))
        mesh.scale.setScalar(0.7 + Math.sin(progress * Math.PI) * 0.6)
      })
    }

    let width = 1
    let height = 1
    const place = () => {
      width = element.clientWidth
      height = element.clientHeight
      if (width === 0 || height === 0) return
      renderer.setSize(width, height)
      composer?.setSize(width, height)
      lineResolution.set(width, height)
      camera.aspect = width / height
      camera.updateProjectionMatrix()
      const visibleHeight = 2 * Math.tan(THREE.MathUtils.degToRad(camera.fov / 2)) * camera.position.z
      const visibleWidth = visibleHeight * camera.aspect
      if (width < 900) {
        graph.position.set(0.1, visibleHeight * 0.19, 0)
        graph.scale.setScalar(Math.min(1.12, visibleWidth / 4.3))
      } else {
        graph.position.set(visibleWidth * 0.245, 0.08, 0)
        graph.scale.setScalar(Math.min(1.3, Math.max(1, visibleWidth / 12)))
      }
    }
    const resizeObserver = new ResizeObserver(place)
    resizeObserver.observe(element)
    place()

    let pointerX = 0
    let pointerY = 0
    const onPointer = (event: PointerEvent) => {
      pointerX = event.clientX / Math.max(1, width) - 0.5
      pointerY = event.clientY / Math.max(1, height) - 0.5
    }
    window.addEventListener('pointermove', onPointer, { passive: true })

    let running = false
    let booted = false
    let heroVisible = true
    let pageVisible = !document.hidden
    let frame = 0
    let startTime = performance.now()
    let pausedAt: number | null = null

    let lastFrame = performance.now()

    const render = (now: number) => {
      if (!running) return
      // A stalled tab must not deliver one enormous step to the smoothers.
      const delta = Math.min((now - lastFrame) / 1000, 0.05)
      lastFrame = now
      const time = (now - startTime) / 1000
      layoutCloud(time)
      layoutGraph(time)
      for (const group of nodeGroups) {
        const { ring, live, pulseSpeed, pulseOffset } = group.userData as { ring: THREE.Mesh; live: boolean; pulseSpeed: number; pulseOffset: number }
        const material = ring.material as THREE.MeshBasicMaterial
        const pulse = (Math.sin(time * pulseSpeed + pulseOffset) + 1) / 2
        material.opacity = (live ? 0.78 : 0.62) + pulse * (live ? 0.22 : 0.2)
      }
      // Ambient drift rides on top of the pointer, so the field keeps breathing when the cursor rests.
      const sway = Math.sin(time * 0.25) * 0.02
      const driftY = Math.sin(time * 0.11) * 0.018
      const driftX = Math.cos(time * 0.085) * 0.012
      const targetY = compact ? sway + driftY : sway + driftY + pointerX * 0.07
      const targetX = compact ? driftX : driftX + pointerY * 0.05
      const ease = 1 - Math.pow(0.02, delta)
      graph.rotation.y += (targetY - graph.rotation.y) * ease
      graph.rotation.x += (targetX - graph.rotation.x) * ease
      if (composer !== null) composer.render()
      else renderer.render(scene, camera)
      if (!still) frame = requestAnimationFrame(render)
      else running = false
    }

    // The clock pauses with the loop, so the entrance plays once and scrolling back does not replay it.
    const start = () => {
      if (!booted || running || !heroVisible || !pageVisible) return
      running = true
      lastFrame = performance.now()
      if (pausedAt === null) startTime = performance.now()
      else startTime += performance.now() - pausedAt
      pausedAt = null
      frame = requestAnimationFrame(render)
    }
    const stop = () => {
      if (!running) return
      running = false
      pausedAt = performance.now()
      cancelAnimationFrame(frame)
    }
    const sync = () => { if (heroVisible && pageVisible) start(); else stop() }
    const hero = document.querySelector('.landing-hero-shell')
    const intersectionObserver = new IntersectionObserver((entries) => {
      const entry = entries.at(0)
      if (entry !== undefined) heroVisible = entry.isIntersecting
      sync()
    }, { threshold: 0.01 })
    intersectionObserver.observe(hero ?? element)
    const onVisibility = () => { pageVisible = !document.hidden; sync() }
    document.addEventListener('visibilitychange', onVisibility)

    const canvas = renderer.domElement
    const onContextLost = (event: Event) => { event.preventDefault(); stop() }
    const onContextRestored = () => sync()
    canvas.addEventListener('webglcontextlost', onContextLost)
    canvas.addEventListener('webglcontextrestored', onContextRestored)

    const boot = () => {
      if (booted) return
      booted = true
      if (still) {
        layoutCloud(SETTLED)
        layoutGraph(SETTLED)
        renderer.render(scene, camera)
        return
      }
      start()
    }
    // Let the browser reach its next paint before the first frame; the frame can be cancelled during
    // React Strict Mode's deliberate mount/unmount check.
    const bootFrame = requestAnimationFrame(boot)

    return () => {
      cancelAnimationFrame(bootFrame)
      stop()
      resizeObserver.disconnect()
      intersectionObserver.disconnect()
      window.removeEventListener('pointermove', onPointer)
      document.removeEventListener('visibilitychange', onVisibility)
      canvas.removeEventListener('webglcontextlost', onContextLost)
      canvas.removeEventListener('webglcontextrestored', onContextRestored)
      scene.traverse((object) => {
        if (object instanceof THREE.Mesh || object instanceof THREE.Line || object instanceof THREE.Points || object instanceof THREE.Sprite) {
          object.geometry?.dispose()
          const materials = Array.isArray(object.material) ? object.material : [object.material]
          materials.forEach((material) => {
            if (material instanceof THREE.SpriteMaterial || material instanceof THREE.PointsMaterial) material.map?.dispose()
            material.dispose()
          })
        }
      })
      lineMaterials.forEach((material) => material.dispose())
      composer?.dispose()
      renderer.dispose()
      renderer.forceContextLoss()
      if (canvas.parentElement === element) element.removeChild(canvas)
    }
  }, [])

  return <div ref={host} className={className} aria-hidden="true" />
}

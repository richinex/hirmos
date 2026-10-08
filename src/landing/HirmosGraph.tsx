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
 * pulses along the causal path. Colour comes from the tokens of the surface it sits on, so on the band it
 * takes the band's ground and ink; under reduced motion the scene renders its final frame once.
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

const MONO = `'Fira Code Variable', 'Fira Code', ui-monospace, monospace`

/** A label drawn on a canvas in the house mono. Call `redraw` once the face has loaded. */
function textSprite(
  text: string,
  colour: THREE.Color,
  width: number,
  fontSize: number,
): THREE.Sprite & { redraw: () => void } {
  const canvas = document.createElement('canvas')
  canvas.width = 512
  canvas.height = 160
  const paint = () => {
    const context = canvas.getContext('2d')
    if (context === null) return
    context.clearRect(0, 0, canvas.width, canvas.height)
    context.fillStyle = `#${colour.getHexString()}`
    context.font = `500 ${fontSize}px ${MONO}`
    context.textAlign = 'center'
    context.textBaseline = 'middle'
    context.fillText(text, canvas.width / 2, canvas.height / 2)
  }
  paint()
  const texture = new THREE.CanvasTexture(canvas)
  texture.colorSpace = THREE.SRGBColorSpace
  texture.minFilter = THREE.LinearFilter
  const sprite = new THREE.Sprite(
    new THREE.SpriteMaterial({ map: texture, transparent: true, depthWrite: false }),
  )
  sprite.scale.set(width, width * (canvas.height / canvas.width), 1)
  return Object.assign(sprite, {
    redraw: () => {
      paint()
      texture.needsUpdate = true
    },
  })
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

function edgeCurve(
  from: THREE.Vector3,
  to: THREE.Vector3,
  bend: number,
): THREE.QuadraticBezierCurve3 {
  const direction = to.clone().sub(from).normalize()
  const start = from.clone().addScaledVector(direction, NODE_RADIUS + 0.06)
  const end = to.clone().addScaledVector(direction, -(NODE_RADIUS + 0.2))
  const control = start.clone().lerp(end, 0.5)
  control.y += bend
  return new THREE.QuadraticBezierCurve3(start, control, end)
}

function arrowHead(
  curve: THREE.QuadraticBezierCurve3,
  colour: THREE.Color,
  size: number,
): THREE.Mesh {
  const mesh = new THREE.Mesh(
    new THREE.ConeGeometry(size * 0.42, size, 18),
    new THREE.MeshBasicMaterial({ color: colour }),
  )
  const endpoint = curve.getPoint(1)
  const tangent = curve.getTangent(1).normalize()
  mesh.position.copy(endpoint).addScaledVector(tangent, size * 0.45)
  mesh.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), tangent)
  return mesh
}

/** The page gutter, linear from 16px at a 400px screen to 48px at 1440px. */
const gutter = (width: number) => Math.min(48, Math.max(16, 0.03077 * width + 3.69))

/**
 * Where the graph may sit, in the scene's own pixels. Beside the copy its centre is level with the centre
 * of the headline and actions, and its height is what that centre allows on both sides; on a narrow
 * screen it sits between the header and the headline.
 */
function stageArea(element: HTMLElement) {
  const band = element.closest('.landing-band')
  const headline = band?.querySelector('.landing-headline')
  const actions = band?.querySelector('.landing-actions')
  const header = band?.querySelector('.landing-header__row')
  if (!band || !headline || !actions || !header) return null
  const frame = element.getBoundingClientRect()
  const text = headline.getBoundingClientRect()
  const buttons = [...actions.children].map((child) => child.getBoundingClientRect())
  const top = header.getBoundingClientRect().bottom - frame.top
  const g = gutter(frame.width)
  if (frame.width < 900)
    return { left: g, top: top + g, right: frame.width - g, bottom: text.top - frame.top - g }
  const copyRight = Math.max(text.right, ...buttons.map((box) => box.right)) - frame.left
  const centre = (text.top + Math.max(...buttons.map((box) => box.bottom))) / 2 - frame.top
  const half = Math.min(centre - (top + g), frame.height - g - centre)
  return { left: copyRight + g, top: centre - half, right: frame.width - g, bottom: centre + half }
}

const easeOut = (t: number) => 1 - (1 - t) ** 3
const easeInOut = (t: number) => (t < 0.5 ? 4 * t ** 3 : 1 - (-2 * t + 2) ** 3 / 2)
const clamp01 = (t: number) => Math.min(1, Math.max(0, t))

export function HirmosGraph({ className }: { readonly className?: string }) {
  const host = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const element = host.current
    if (element === null) return

    const style = getComputedStyle(element)
    const stage = colourToken(style, '--color-stage', '#0A0A0B')
    const ink = colourToken(style, '--color-ink', '#E6E3DC')
    const bone = colourToken(style, '--color-bone', '#B8B5AE')
    const muted = colourToken(style, '--color-muted', '#8C8A85')
    const signal = colourToken(style, '--color-signal', '#E4501F')
    const dark = stage.getHSL({ h: 0, s: 0, l: 0 }).l < 0.5
    const still = window.matchMedia('(prefers-reduced-motion: reduce)').matches
    const compact = window.matchMedia('(max-width: 900px)').matches

    const scene = new THREE.Scene()
    // The fog was tuned at a camera distance of 9. Exponential-squared fog depends on density times
    // distance, so the density is rescaled wherever the framing moves the camera.
    const FOG_AT_NINE = dark ? 0.055 : 0.042
    const fog = new THREE.FogExp2(stage.getHex(), FOG_AT_NINE)
    scene.fog = fog
    const camera = new THREE.PerspectiveCamera(36, 1, 0.1, 40)
    camera.position.set(0, 0, 9)

    let renderer: THREE.WebGLRenderer
    try {
      renderer = new THREE.WebGLRenderer({
        alpha: true,
        antialias: true,
        powerPreference: 'low-power',
      })
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
    const nodePositions = new Map<GraphNode['id'], THREE.Vector3>(
      NODES.map((node) => [node.id, point(node.point)]),
    )
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
      const tint = (owner.id === 'X' || owner.id === 'Y' ? signal : bone)
        .clone()
        .lerp(stage, dark ? 0.15 : 0.05)
      colours[index * 3] = tint.r
      colours[index * 3 + 1] = tint.g
      colours[index * 3 + 2] = tint.b
    }
    const cloudGeometry = new THREE.BufferGeometry()
    const positionAttribute = new THREE.BufferAttribute(positions, 3)
    positionAttribute.setUsage(THREE.DynamicDrawUsage)
    cloudGeometry.setAttribute('position', positionAttribute)
    cloudGeometry.setAttribute('color', new THREE.BufferAttribute(colours, 3))
    const cloud = new THREE.Points(
      cloudGeometry,
      new THREE.PointsMaterial({
        map: discTexture(),
        vertexColors: true,
        size: compact ? 0.06 : 0.07,
        sizeAttenuation: true,
        transparent: true,
        opacity: dark ? 0.55 : 0.62,
        depthWrite: false,
      }),
    )
    graph.add(cloud)

    // The variables: flat discs so the glyph sits in front, a ring in the node's tone, the caption below.
    const nodeGroups: THREE.Group[] = []
    const labels: { redraw: () => void }[] = []
    for (const node of NODES) {
      const group = new THREE.Group()
      group.position.copy(nodePositions.get(node.id) ?? new THREE.Vector3())
      const live = node.id === 'X' || node.id === 'Y'
      const disc = new THREE.Mesh(
        new THREE.CircleGeometry(NODE_RADIUS, 64),
        new THREE.MeshBasicMaterial({ color: stage.clone().lerp(ink, dark ? 0.06 : 0.02) }),
      )
      const ring = new THREE.Mesh(
        new THREE.RingGeometry(NODE_RADIUS, NODE_RADIUS + 0.018, 96),
        new THREE.MeshBasicMaterial({
          color: live ? signal : bone,
          transparent: true,
          opacity: live ? 0.95 : 0.8,
        }),
      )
      ring.position.z = 0.002
      const glyph = textSprite(node.id, live ? signal : ink, 0.4, 118)
      glyph.position.z = 0.03
      const caption = textSprite(node.caption, muted, 1.15, 26)
      labels.push(glyph, caption)
      caption.position.set(0, -(NODE_RADIUS + 0.22), 0.03)
      group.add(disc, ring, glyph, caption)
      group.scale.setScalar(0)
      group.userData = {
        ring,
        live,
        pulseSpeed: 0.42 + Math.random() * 0.34,
        pulseOffset: Math.random() * Math.PI * 2,
      }
      graph.add(group)
      nodeGroups.push(group)
    }

    // The arrows: fat lines with a constant pixel width, drawn on segment by segment.
    const lineResolution = new THREE.Vector2(1, 1)
    const lineMaterials: LineMaterial[] = []
    const edges: Array<{
      readonly line: Line2
      readonly head: THREE.Mesh
      readonly curve: THREE.QuadraticBezierCurve3
      readonly emphasis: GraphEdge['emphasis']
    }> = []
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
    const pulses = edges
      .filter((edge) => edge.emphasis === 'effect')
      .map((edge, index) => {
        const mesh = new THREE.Mesh(
          new THREE.SphereGeometry(0.045, 16, 10),
          new THREE.MeshBasicMaterial({ color: signal }),
        )
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
    const NODE_DURATION = 0.6

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
      nodeGroups.forEach((group, index) => {
        const progress = still ? 1 : clamp01((time - NODE_AT - index * NODE_STEP) / NODE_DURATION)
        group.scale.setScalar(Math.max(0.0001, easeOut(progress)))
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
        curve.getPoint(progress, mesh.position)
        mesh.scale.setScalar(0.7 + Math.sin(progress * Math.PI) * 0.6)
      })
    }

    // The graph's settled extent: the nodes, the cloud around them and the captions below.
    const extent = new THREE.Box3()
    for (const node of NODES) extent.expandByPoint(point(node.point))
    extent.expandByScalar(NODE_RADIUS + 0.55)
    const focus = extent.getCenter(new THREE.Vector3())
    const size = extent.getSize(new THREE.Vector3())

    let width = 1
    let height = 1
    let distance = camera.position.z
    const place = () => {
      width = element.clientWidth
      height = element.clientHeight
      if (width === 0 || height === 0) return
      renderer.setSize(width, height)
      composer?.setSize(width, height)
      lineResolution.set(width, height)
      camera.aspect = width / height
      const area = stageArea(element) ?? { left: width / 2, top: 0, right: width, bottom: height }
      // A box w by h at distance d projects to w f / d by h f / d, with f the focal length in pixels.
      const focal = height / 2 / Math.tan(THREE.MathUtils.degToRad(camera.fov / 2))
      distance = Math.max(
        (size.x * focal) / (area.right - area.left),
        (size.y * focal) / (area.bottom - area.top),
      )
      fog.density = (FOG_AT_NINE * 9) / distance
      // The narrow-screen scrim fades in just above the headline, measured rather than guessed.
      const band = element.closest<HTMLElement>('.landing-band')
      const headline = band?.querySelector('.landing-headline')
      if (band && headline)
        band.style.setProperty('--hero-copy-top', `${Math.round(headline.getBoundingClientRect().top - band.getBoundingClientRect().top)}px`)
      // Move the projection centre onto the centre of the stage area.
      const middleX = (area.left + area.right) / 2
      const middleY = (area.top + area.bottom) / 2
      camera.setViewOffset(width, height, width / 2 - middleX, height / 2 - middleY, width, height)
      camera.updateProjectionMatrix()
    }
    const resizeObserver = new ResizeObserver(place)
    resizeObserver.observe(element)
    const copy = element.closest('.landing-band')?.querySelector('.landing-hero-copy')
    if (copy) resizeObserver.observe(copy)
    place()
    // The labels and the copy's measured box both depend on the faces, so both settle once they load.
    let disposed = false
    void document.fonts.load(`500 118px ${MONO}`).then(() => {
      if (disposed) return
      labels.forEach((label) => label.redraw())
      place()
      if (still) renderer.render(scene, camera)
    })

    // A fine pointer leans the view: one damped target, split so the camera shifts, its aim is carried 42%
    // of the way and the graph turns by a smaller amount. Touch, a coarse pointer and reduced motion keep
    // the centred pose; leaving the page returns it to centre.
    let pointerX = 0
    let pointerY = 0
    const finePointer = !still && window.matchMedia('(pointer: fine)').matches
    const onPointer = (event: PointerEvent) => {
      if (event.pointerType === 'touch') return
      pointerX = event.clientX / Math.max(1, window.innerWidth) - 0.5
      pointerY = event.clientY / Math.max(1, window.innerHeight) - 0.5
    }
    const onPointerLeave = () => {
      pointerX = 0
      pointerY = 0
    }
    if (finePointer) {
      window.addEventListener('pointermove', onPointer, { passive: true })
      document.documentElement.addEventListener('pointerleave', onPointerLeave)
    }
    let leanX = 0
    let leanY = 0
    const aim = new THREE.Vector3()
    const frameCamera = () => {
      const shiftX = -leanX * 0.7
      const shiftY = leanY * 0.44
      camera.position.set(focus.x + shiftX, focus.y + shiftY, focus.z + distance)
      camera.lookAt(aim.set(focus.x + shiftX * 0.42, focus.y + shiftY * 0.42, focus.z))
    }

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
      const delta = Math.min((now - lastFrame) / 1000, 1 / 30)
      lastFrame = now
      const time = (now - startTime) / 1000
      layoutCloud(time)
      layoutGraph(time)
      for (const group of nodeGroups) {
        const { ring, live, pulseSpeed, pulseOffset } = group.userData as {
          ring: THREE.Mesh
          live: boolean
          pulseSpeed: number
          pulseOffset: number
        }
        const material = ring.material as THREE.MeshBasicMaterial
        const pulse = (Math.sin(time * pulseSpeed + pulseOffset) + 1) / 2
        material.opacity = (live ? 0.78 : 0.62) + pulse * (live ? 0.22 : 0.2)
      }
      // Ambient drift rides on top of the pointer, so the field keeps breathing when the cursor rests.
      const follow = 1 - Math.pow(1 - 0.055, delta * 60)
      leanX += (pointerX - leanX) * follow
      leanY += (pointerY - leanY) * follow
      const sway = Math.sin(time * 0.25) * 0.02
      const driftY = Math.sin(time * 0.11) * 0.018
      const driftX = Math.cos(time * 0.085) * 0.012
      graph.rotation.y = sway + driftY + leanX * 0.055
      graph.rotation.x = driftX + leanY * 0.026
      frameCamera()
      if (composer !== null) composer.render()
      else renderer.render(scene, camera)
      if (!still) frame = requestAnimationFrame(render)
      else running = false
    }

    // The clock pauses with the loop, so the entrance plays once and scrolling back does not replay it.
    const start = () => {
      if (still || !booted || running || !heroVisible || !pageVisible) return
      running = true
      element.dataset.animationActive = 'true'
      lastFrame = performance.now()
      if (pausedAt === null) startTime = performance.now()
      else startTime += performance.now() - pausedAt
      pausedAt = null
      frame = requestAnimationFrame(render)
    }
    const stop = () => {
      if (!running) return
      running = false
      element.dataset.animationActive = 'false'
      pausedAt = performance.now()
      cancelAnimationFrame(frame)
    }
    const sync = () => {
      if (heroVisible && pageVisible) start()
      else stop()
    }
    const hero = document.querySelector('.landing-band')
    const intersectionObserver = new IntersectionObserver(
      (entries) => {
        const entry = entries.at(0)
        if (entry !== undefined) heroVisible = entry.isIntersecting
        sync()
      },
      { threshold: 0.01 },
    )
    intersectionObserver.observe(hero ?? element)
    const onVisibility = () => {
      pageVisible = !document.hidden
      sync()
    }
    document.addEventListener('visibilitychange', onVisibility)

    const canvas = renderer.domElement
    const onContextLost = (event: Event) => {
      event.preventDefault()
      stop()
    }
    const onContextRestored = () => sync()
    canvas.addEventListener('webglcontextlost', onContextLost)
    canvas.addEventListener('webglcontextrestored', onContextRestored)

    const boot = () => {
      if (booted) return
      booted = true
      if (still) {
        layoutCloud(SETTLED)
        layoutGraph(SETTLED)
        frameCamera()
        renderer.render(scene, camera)
        return
      }
      start()
    }
    // Let the browser reach its next paint before the first frame; the frame can be cancelled during
    // React Strict Mode's deliberate mount/unmount check.
    const bootFrame = requestAnimationFrame(boot)
    // Development builds expose where each node lands on screen, so the framing can be checked by measurement.
    if (import.meta.env.DEV)
      Object.assign(element, {
        nodesOnScreen: () =>
          nodeGroups.map((group, index) => {
            const at = group.getWorldPosition(new THREE.Vector3()).project(camera)
            return [NODES[index]?.id, Math.round(((at.x + 1) / 2) * width), Math.round(((1 - at.y) / 2) * height)]
          }),
      })

    return () => {
      cancelAnimationFrame(bootFrame)
      stop()
      resizeObserver.disconnect()
      intersectionObserver.disconnect()
      disposed = true
      window.removeEventListener('pointermove', onPointer)
      document.documentElement.removeEventListener('pointerleave', onPointerLeave)
      document.removeEventListener('visibilitychange', onVisibility)
      canvas.removeEventListener('webglcontextlost', onContextLost)
      canvas.removeEventListener('webglcontextrestored', onContextRestored)
      scene.traverse((object) => {
        if (
          object instanceof THREE.Mesh ||
          object instanceof THREE.Line ||
          object instanceof THREE.Points ||
          object instanceof THREE.Sprite
        ) {
          object.geometry?.dispose()
          const materials = Array.isArray(object.material) ? object.material : [object.material]
          materials.forEach((material) => {
            if (
              material instanceof THREE.SpriteMaterial ||
              material instanceof THREE.PointsMaterial
            )
              material.map?.dispose()
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

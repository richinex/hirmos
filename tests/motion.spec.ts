import {test,expect} from '@playwright/test'

for(const reduced of [false,true])test('motion primitives '+(reduced?'respect reduced motion':'retain normal movement'),async({page},info)=>{
  await page.emulateMedia({reducedMotion:reduced?'reduce':'no-preference'})
  await page.goto('/app')
  await page.evaluate(async()=>{const m=await import(new URL('/tests/motion-harness.tsx',location.href).href);m.mount()})
  const host=page.locator('#motion-harness')
  await expect(host.getByRole('button',{name:'Zoom in',exact:true})).toBeVisible()
  const durations=()=>page.evaluate(async()=>{const m=await import(new URL('/src/lib/motion.ts',location.href).href);return ['zoom','fit','arrange'].map(k=>m.canvasMotion(k).duration)})
  expect(await durations()).toEqual(reduced?[0,0,0]:[160,220,320])
  for(const action of ['Zoom in','Zoom out','Fit the canvas']){
    const distinct=await host.evaluate(async(el,action)=>{
      const viewport=el.querySelector<HTMLElement>('.react-flow__viewport')!
      const values=new Set<string>()
      const observer=new MutationObserver(()=>values.add(viewport.style.transform))
      observer.observe(viewport,{attributes:true,attributeFilter:['style']})
      el.querySelector<HTMLButtonElement>('button[aria-label="'+action+'"]')!.click()
      await new Promise(resolve=>setTimeout(resolve,350))
      observer.disconnect();return values.size
    },action)
    if(reduced)expect(distinct).toBeLessThanOrEqual(2)
    else expect(distinct).toBeGreaterThan(2)
  }
  await host.getByRole('radio',{name:'Second',exact:true}).check()
  await expect(host.getByRole('radio',{name:'Second',exact:true})).toBeChecked()
  const movingProperties=await host.locator('.segment-knob').evaluate(el=>getComputedStyle(el).transitionProperty)
  if(reduced)expect(movingProperties).not.toMatch(/left|right|top|bottom|transform/)
  else expect(movingProperties).toContain('left')
  const orb=host.getByRole('img',{name:'Analysis running'})
  await expect(orb).toBeVisible()
  const frame=()=>orb.evaluate(el=>(el as HTMLCanvasElement).toDataURL())
  const before=await frame()
  await page.waitForTimeout(180)
  if(reduced)expect(await frame()).toBe(before)
  else expect(await frame()).not.toBe(before)
  const loop=await host.locator('.pulse-live').evaluate(el=>{const s=getComputedStyle(el);return {duration:parseFloat(s.animationDuration),iterations:s.animationIterationCount}})
  if(reduced){expect(loop.duration).toBeLessThan(0.001);expect(loop.iterations).toBe('1')}
  else expect(loop.iterations).toBe('infinite')
  for(const theme of ['light','dark']){
    await page.evaluate(theme=>document.documentElement.dataset.theme=theme,theme)
    await page.screenshot({path:info.outputPath((reduced?'reduced':'normal')+'-'+theme+'.png')})
  }
  // Preferences are read at the action, not captured when the canvas mounted.
  await page.emulateMedia({reducedMotion:reduced?'no-preference':'reduce'})
  expect(await durations()).toEqual(reduced?[160,220,320]:[0,0,0])
})

test('raw CSS transitions use the house easing',async({page})=>{
  await page.goto('/')
  await expect(page.locator('.landing-primary').first()).toBeVisible()
  expect(await page.locator('.landing-primary').first().evaluate(el=>getComputedStyle(el).transitionTimingFunction)).toContain('cubic-bezier(0.2, 0.7, 0.2, 1)')
})

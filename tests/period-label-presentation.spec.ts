import {test,expect} from '@playwright/test'

test('Bacon comparisons and removed periods display recorded years without code fallbacks',async({page},info)=>{
  await page.goto('/app')
  await page.evaluate(async()=>{
    const {default:React}=await import(new URL('/node_modules/.vite/deps/react.js',location.href).href)
    const {default:{createRoot}}=await import(new URL('/node_modules/.vite/deps/react-dom_client.js',location.href).href)
    const {BaconResult}=await import(new URL('/src/components/time-series/PanelRegressionResult.tsx',location.href).href)
    const {StaggeredDidResult}=await import(new URL('/src/components/estimation/StaggeredDidResult.tsx',location.href).href)
    const {periodLabel}=await import(new URL('/src/domain/periodLabels.ts',location.href).href)
    let refused=false
    try{periodLabel(24,new Map())}catch{refused=true}
    if(!refused)throw Error('An absent source label was silently replaced.')
    const interval={kind:'estimated',estimate:1,standardError:0.1,lower:0.8,upper:1.2}
    const family={keys:[0],intervals:[interval],coverage:{kind:'pointwise'},analyticalCovariance:[[0.01]]}
    const sourcePeriods=Array.from({length:30},(_,code)=>({code,label:String(1980+code)}))
    const evidence={times:[0,1],units:['a','b'],clusterCount:2,weighted:false,overall:{simple:{...interval,estimate:68.3371824685026,lower:60,upper:75}},specification:{confidence:0.95},events:family,cohorts:family,calendar:family,cells:{...family,keys:[[0,1]]},support:[{cohorts:[0],treatedUnits:1}],changes:[{kind:'noUntreatedComparison',period:24},{kind:'noUntreatedComparison',period:29}],smallCohorts:[],fits:[]}
    const run={outcome:{name:'Outcome'},periods:sourcePeriods.map(p=>p.label),evidence:{twfe:1,reconstructed:1,decomposition:{kind:'unadjusted',components:[{comparison:{kind:'earlierVsLater',earlier:6,later:12},estimate:1,weight:1}]}}}
    const host=document.createElement('div');host.id='period-label-test';document.body.replaceChildren(host)
    createRoot(host).render(React.createElement('div',null,React.createElement(BaconResult,{run}),React.createElement(StaggeredDidResult,{evidence,labels:['1980','1981'],sourcePeriods})))
  })
  await expect(page.getByText('1986 vs 1992: earlier vs later treated',{exact:true})).toBeVisible()
  const changes=page.getByRole('table',{name:'Panel preparation'})
  await expect(changes.getByText('2004',{exact:true})).toBeVisible()
  await expect(changes.getByText('2009',{exact:true})).toBeVisible()
  await expect(page.getByText('6 vs 12: earlier vs later treated',{exact:true})).toHaveCount(0)
  await expect(page.getByText(/Adoption codes follow/)).toHaveCount(0)
  const simple=page.getByRole('table',{name:'Simple ATT',exact:true})
  await expect(simple).toBeVisible()
  const formatted=await page.evaluate(async()=>{
    const {formatStatistic}=await import(new URL('/src/lib/format/number.ts',location.href).href)
    return formatStatistic('raw',68.3371824685026).text
  })
  await expect(simple.getByRole('cell').first()).toHaveText(formatted)
  await expect(page.getByText(/Bounds are 95% pointwise confidence intervals/)).toBeVisible()
  await page.screenshot({path:info.outputPath('source-year-labels.png'),fullPage:true})
})

import {test,expect} from '@playwright/test'
import {sampleMembershipSchema} from '../src/domain/sampleMembership'

test('membership sets are disjoint, explicit and preserve numeric records',()=>{
  expect(sampleMembershipSchema.parse({column:'site',experimental:1,observational:0}).kind).toBe('numeric')
  const sample={kind:'categories',column:'site',experimental:['Riverside'],observational:["O'Brien",'Alameda','Los Angeles']}
  expect(sampleMembershipSchema.safeParse(sample).success).toBe(true)
  for(const delta of [{experimental:[]},{observational:[]},{observational:['Riverside']},{experimental:['Riverside','Riverside']}])expect(sampleMembershipSchema.safeParse({...sample,...delta}).success).toBe(false)
})

test('categorical materialization reuses filters without dropping or reordering source rows',async({page})=>{
  await page.goto('/app')
  const result=await page.evaluate(async()=>{
    const data=await import(new URL('/src/data/client.ts',location.href).href)
    const workflow=await import(new URL('/src/domain/workflow.ts',location.href).href)
    const samples=await import(new URL('/src/domain/surrogateSamples.ts',location.href).href)
    const prep=await import(new URL('/src/data/surrogateInput.ts',location.href).href)
    const csv="site,w,y,s\nRiverside,0,,0\nO'Brien,,1,0\nRiverside,1,,1\nAlameda,,3,1\nRiverside,0,,2\nLos Angeles,,5,2\nRiverside,1,,3\nUnselected,,,,\nO'Brien,,7,3\n"
    // No coercion of named sites into numbers in the input file.
    const file=new File([csv.replace('Unselected,,,,','Unselected,,,')],'sites.csv',{type:'text/csv'})
    const p=await data.profileSourceInWorker(workflow.newImportRequestId(),file)
    if(!p.ok)throw Error(JSON.stringify(p.error))
    const id=(name:string)=>p.value.columns.find((c:{name:string})=>c.name===name).id
    const selection=samples.surrogateSelectionSchema.parse({sample:{kind:'categories',column:id('site'),experimental:['Riverside'],observational:["O'Brien",'Alameda','Los Angeles']},treatment:id('w'),outcome:id('y'),surrogates:[id('s')],adjustment:{kind:'none'},estimator:'index',uncertainty:{kind:'none'}})
    const source=workflow.selectSource(file)
    if(!source.ok)throw Error(JSON.stringify(source.error))
    const selected=await prep.prepareSurrogateSamples(source.value,p.value,selection)
    const matrix=await data.materializeNumericColumnsInWorker(file,p.value,samples.surrogateSelectionColumns(selection),selection.sample)
    if(!matrix.ok)throw Error(JSON.stringify(matrix.error))
    const invalid=matrix.value.validity.slice();invalid[0]&=~1
    const missing=samples.selectSurrogateSamples({...matrix.value,validity:invalid},selection,p.value)
    return {selected,missing}
  })
  expect(result.selected.ok,JSON.stringify(result.selected)).toBe(true)
  expect(result.selected.value.rows).toEqual({experimental:[0,2,4,6],observational:[1,3,5,8],excluded:[7]})
  expect(result.selected.value.request.observational.outcome).toEqual([1,3,5,7])
  expect(result.missing.error).toMatchObject({kind:'missing-value',row:0,sample:'membership'})
})

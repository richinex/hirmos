import {test,expect} from '@playwright/test'

test('membership category lookup includes all source values beyond the profile limit',async({page})=>{
  await page.goto('/app')
  const result=await page.evaluate(async()=>{
    const data=await import(new URL('/src/data/client.ts',location.href).href)
    const workflow=await import(new URL('/src/domain/workflow.ts',location.href).href)
    const file=new File(['site,y\n'+Array.from({length:45},(_,i)=>'Site '+i+','+i).join('\n')],'many-sites.csv',{type:'text/csv'})
    const profile=await data.profileSourceInWorker(workflow.newImportRequestId(),file)
    if(!profile.ok)throw Error(JSON.stringify(profile.error))
    const column=profile.value.columns.find((c:{name:string})=>c.name==='site').id
    const summary=await data.summarizeColumnsInWorker(file,profile.value,column)
    if(!summary.ok)throw Error(JSON.stringify(summary.error))
    return summary.value.columns.find((c:{column:string})=>c.column===column).categories.map((c:{value:string})=>c.value)
  })
  expect(result).toHaveLength(45)
  expect(result).toEqual(expect.arrayContaining(Array.from({length:45},(_,i)=>'Site '+i)))
})

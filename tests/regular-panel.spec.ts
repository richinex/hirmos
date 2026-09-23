import {test,expect} from '@playwright/test'

test('regular panels require validated columns, complete keys and an actual calendar alignment',async({page})=>{
  test.setTimeout(120_000)
  await page.goto('/app')
  const actual=await page.evaluate(async()=>{
    const data=await import(new URL('/src/data/client.ts',location.href).href)
    const workflow=await import(new URL('/src/domain/workflow.ts',location.href).href)
    const adapter=await import(new URL('/src/data/regularPanelInput.ts',location.href).href)
    const domain=await import(new URL('/src/domain/regularPanel.ts',location.href).href)
    async function check(times:string[],frequency:string,change='none') {
      let rows=['a','b'].flatMap(unit=>times.map(time=>`${unit},${time},1`))
      if(change==='duplicate')rows[1]=rows[0]!
      if(change==='unbalanced')rows=rows.slice(1)
      const file=new File([['unit,time,value',...rows].join('\n')],'panel.csv',{type:'text/csv'})
      const result=await data.profileSourceInWorker(workflow.newImportRequestId(),file)
      if(!result.ok)throw Error(JSON.stringify(result.error))
      const profile=result.value
      const column=(name:string)=>profile.columns.find((c:{name:string})=>c.name===name)
      const source=workflow.selectSource(file)
      if(!source.ok)throw Error('Invalid test source')
      const prepared={kind:'prepared-panel',sampling:{kind:'regular-panel',unitColumn:column('unit').id,timeColumn:change==='missing-column'?'absent':column('time').id,frequency}}
      const matrix={rowCount:change==='row-count'?rows.length-1:rows.length,columns:[{id:column('value').id,name:'value'}],values:new Float64Array(rows.length).fill(1),imputedCells:[],leadingRowsRemoved:0,timeAxis:null}
      const panel=await adapter.prepareRegularPanel(source.value,profile,prepared,matrix,()=>change!=='cancelled')
      if(!panel.ok)return panel.error.kind==='structure'?panel.error.problem.kind:panel.error.kind
      const lag=domain.lagDesign(panel.value,[{column:0,lag:1,name:'value lag1'}],[0])
      if(lag.keys.length!==rows.length)throw Error('Lost key provenance')
      return panel.value.clock.kind==='ordinal'?'ordinal':panel.value.clock.schedule
    }
    return {
      ordinal:await check(['1','2','3'],'weekly'),
      ordinalGap:await check(['1','3','4'],'weekly'),
      missingColumn:await check(['1','2','3'],'weekly','missing-column'),
      duplicate:await check(['1','2','3'],'weekly','duplicate'),
      unbalanced:await check(['1','2','3'],'weekly','unbalanced'),
      rowCount:await check(['1','2','3'],'weekly','row-count'),
      cancelled:await check(['1','2','3'],'weekly','cancelled'),
      daily:await check(['2024-02-28','2024-02-29','2024-03-01'],'daily'),
      dailyGap:await check(['2024-02-28','2024-03-01'],'daily'),
      weekly:await check(['2024-01-02','2024-01-09','2024-01-16'],'weekly'),
      weeklyGap:await check(['2024-01-02','2024-01-16'],'weekly'),
      monthly:await check(['2024-01-31','2024-02-29','2024-03-31'],'monthly'),
      monthlyMiddle:await check(['2024-01-15','2024-02-15','2024-03-15'],'monthly'),
      mixed:await check(['2024-01-01','2024-02-29','2024-03-31'],'monthly'),
      quarter:await check(['2024-01-01','2024-04-01','2024-07-01'],'quarterly'),
      year:await check(['2022-12-31','2023-12-31','2024-12-31'],'yearly'),
    }
  })
  expect(actual).toEqual({ordinal:'ordinal',ordinalGap:'ordinal-gap',missingColumn:'column-missing',duplicate:'duplicate-panel-cell',unbalanced:'missing-panel-cell',rowCount:'row-count-mismatch',cancelled:'cancelled',daily:'daily',dailyGap:'calendar-gaps',weekly:'tuesday',weeklyGap:'calendar-gaps',monthly:'month-end',monthlyMiddle:'calendar-alignment',mixed:'calendar-alignment',quarter:'quarter-start',year:'year-end'})
})

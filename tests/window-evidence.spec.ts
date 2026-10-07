import { expect, test } from '@playwright/test'
import { choose, chapter } from './examples/support'

test('calendar edges preserve spans, lineage and unavailable states', async ({page}) => {
  await page.goto('/app')
  const result=await page.evaluate(async () => {
    const m=await import(new URL('/src/domain/windowEvidence.ts',location.href).href)
    const make=(span:string, window:unknown={kind:'year-end'}, tail:unknown[]=[]):any => {
      const blocks=[{kind:'input',file:{kind:'empty'}},{kind:'calendar-events',column:'date',interpretation:{kind:'timestamp'},span,window,name:'holiday_share'},...tail,{kind:'output'}]
      return {kind:'pipeline-derived',inputs:[],graph:{nodes:blocks.map((block,i)=>({id:String(i),block,position:{x:0,y:i}})),edges:blocks.slice(1).map((_,i)=>({from:String(i),to:String(i+1),port:0}))}}
    }
    const edge=(span:string,date:string, window?:unknown)=>m.calendarEdgeFor(make(span,window),'date',Date.parse(date),span)
    const renamed=make('week',undefined,[{kind:'select-columns',mode:'keep',columns:['date','holiday_share'],renames:[{from:'date',to:'week'}]}])
    const blocked=make('week',undefined,[{kind:'script',code:'result = df'}])
    const overwritten=make('week',undefined,[{kind:'derive-columns',columns:[{name:'date',expression:"DATE '2024-01-01'"}]}])
    const first=edge('week','2023-12-25')
    return {
      first,day:edge('day','2024-01-02'),outside:edge('week','2024-01-01'),month:edge('month','2023-12-01'),
      leap:edge('day','2024-02-28',{kind:'custom',from:{day:28,month:2},to:{day:1,month:3}}),
      leapEnd:m.rowCoverageEnd(Date.parse('2024-01-31'),'month'),
      fullYear:edge('day','2023-12-31',{kind:'custom',from:{day:1,month:1},to:{day:31,month:12}}),
      nonLeap:edge('day','2023-02-28',{kind:'custom',from:{day:1,month:2},to:{day:29,month:2}}),
      renamed:m.calendarEdgeFor(renamed,'week',Date.parse('2023-12-25'),'week'),
      blocked:m.calendarEdgeFor(blocked,'date',Date.parse('2023-12-25'),'week'),
      overwritten:m.calendarEdgeFor(overwritten,'date',Date.parse('2023-12-25'),'week'),
      capped:m.calendarEdgeFor(make('day'),'date',Date.parse('2024-01-08'),'day',Date.parse('2024-01-01')),
      mismatch:m.calendarEdgeFor(make('day'),'date',Date.parse('2024-01-08'),'day',null,{kind:'date-format',format:'%d/%m/%Y'}),
      tampered:m.calendarEdgeSchema.safeParse({...first,windows:first.windows.map((w:any)=>({...w,endsInside:false}))}).success,
      trim:m.trimmingFact({kind:'window',start:8,endExclusive:58,sourceRows:60}),
      season:m.seasonalFact(50,52),
      invalid:m.windowContextSchema.safeParse({completeInterval:{kind:'window',start:5,endExclusive:4,sourceRows:8},calendar:first}).success,
    }
  })
  expect(result.first.lastCoveredDay).toBe(Date.parse('2023-12-31'))
  expect(result.first.windows[0].endsInside).toBe(true)
  expect(result.day.windows[0].endsInside).toBe(false)
  expect(result.outside.windows[0].endsInside).toBe(false)
  expect(result.month.windows[0].endsInside).toBe(true)
  expect(result.leap.windows[0].endsInside).toBe(true)
  expect(result.leapEnd).toBe(Date.parse('2024-02-29'))
  expect(result.fullYear.windows[0].endsInside).toBe(false)
  expect(result.nonLeap.windows[0].endsInside).toBe(false)
  expect(result.renamed).toEqual(result.first)
  expect(result.blocked).toEqual({kind:'unavailable',reason:'untraced-date'})
  expect(result.overwritten).toEqual(result.blocked)
  expect(result.mismatch).toEqual(result.blocked)
  expect(result.capped.lastCoveredDay).toBe(Date.parse('2023-12-31'))
  expect(result.capped.windows[0].endsInside).toBe(true)
  expect(result.tampered).toBe(false)
  expect(result.invalid).toBe(false)
  expect(result.trim).toContain('kept 50 of 60 source rows')
  expect(result.season).toContain('0.94 cycles')
})

test('saved ITS shows trimming and seasonal span without blocking the fit',async ({page},info)=>{
  test.setTimeout(180000)
  const csv='date,y\n'+Array.from({length:60},(_,i)=>`${new Date(Date.UTC(2022,10,21+7*i)).toISOString().slice(0,10)},${i<8||i>=58?'':10+i*.2+Math.sin(i*.7)+(i>=32?2:0)}`).join('\n')
  const file={name:'window.csv',mimeType:'text/csv',buffer:Buffer.from(csv)}
  await page.goto('/app')
  await page.getByRole('textbox',{name:'Project name'}).fill('Window evidence')
  await page.getByRole('button',{name:'Create project',exact:true}).click()
  await page.locator('input[type=file]').setInputFiles(file)
  await page.getByRole('button',{name:'Inspect data',exact:true}).click()
  await page.getByRole('radio',{name:/Regular time series/}).click()
  await choose(page,'Time column','date')
  await choose(page,'Source frequency','Weekly')
  await page.getByRole('checkbox',{name:'y',exact:true}).check()
  await page.getByRole('radio',{name:'Complete contiguous interval',exact:true}).click()
  await page.getByRole('button',{name:/Create prepared dataset/}).click()
  await expect(page.getByRole('heading',{name:'Next steps',exact:true})).toBeVisible()
  await chapter(page,/Time-series analysis/)
  await page.getByRole('radio',{name:'Interrupted series',exact:true}).click()
  await choose(page,'Series','y')
  await page.getByRole('spinbutton',{name:'Intervention row',exact:true}).fill('25')
  await page.getByRole('button',{name:'Fit interrupted series',exact:true}).click()
  const result=page.getByRole('region',{name:'Time-series result'}).filter({visible:true})
  await expect(result).toBeVisible({timeout:60000})
  const window=result.getByRole('region',{name:'Analysis window'})
  await expect(window).toContainText('kept 50 of 60 source rows')
  await expect(window).toContainText('removed 8 before and 2 after')
  await expect(window).toContainText('0.94 cycles')
  await window.scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('window-evidence.png')})
  await chapter(page,/Projects/)
  await expect(page.getByRole('button',{name:'Open Window evidence',exact:true})).toBeVisible()
  await page.reload()
  await page.getByRole('button',{name:'Open Window evidence',exact:true}).click()
  await expect(page.getByRole('heading',{name:'Choose the data file again',exact:true})).toBeVisible()
  await page.locator('input[type=file]').setInputFiles(file)
  await chapter(page,/Time-series analysis/)
  await page.getByRole('radio',{name:'Interrupted series',exact:true}).click()
  await expect(result).toContainText('0.94 cycles')
  await expect(result).toContainText('kept 50 of 60 source rows')
})

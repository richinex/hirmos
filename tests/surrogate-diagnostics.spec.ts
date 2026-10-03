import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { z } from 'zod'
import { chapter, choose, prepare } from './examples/support'
import { formatStatistic } from '../src/lib/format/number'
import { surrogateDiagnosticRequestSchema, surrogateDiagnosticEvidenceSchema } from '../src/domain/surrogateDiagnostics'

const matrix = z.array(z.array(z.number()))
const baseSchema = z.object({ experimental: matrix, observational: matrix, treatment: z.array(z.number()), outcome: z.array(z.number()),
  experimental_outcome:z.array(z.number()), propensity_design:matrix.nullable(), observational_baseline:matrix.nullable(),
  validation_surrogacy:z.array(z.number()),validation_comparability:z.array(z.number()) })
const fixture = z.object({cases:z.record(z.string(),z.object({base:baseSchema}))}).parse(JSON.parse(readFileSync(new URL('../../octopus/rust-causal-transpile/oracle/surrogate-index/fixtures.json',import.meta.url),'utf8')))
const bases = Object.values(fixture.cases).map(c=>c.base)
function samples(b:z.infer<typeof baseSchema>) {
  const columns = (b.propensity_design?.[0]?.length ?? 1)-1
  const predictors = (rows:number[][])=>rows.map(row=>row.slice(1,row.length-columns))
  return {experimental:{surrogates:predictors(b.experimental),treatment:b.treatment},observational:{surrogates:predictors(b.observational),outcome:b.outcome},
    adjustment:b.propensity_design===null?{kind:'none'}:{kind:'baseline',experimental:b.propensity_design.map(row=>row.slice(1)),observational:b.observational_baseline!.map(row=>row.slice(1))}}
}

test('validation regressions match R through the browser worker',async({page})=>{
  await page.goto('/app')
  for(const b of bases){
    const request=surrogateDiagnosticRequestSchema.parse({...samples(b),analysis:{kind:'validation',experimentalOutcome:b.experimental_outcome}})
    const result=await page.evaluate(async model=>{
      const client=await import(new URL('/src/analysis/client.ts',location.href).href)
      return client.runSurrogateDiagnostics(model)
    },request)
    expect(result.ok,JSON.stringify(result)).toBe(true)
    const evidence=surrogateDiagnosticEvidenceSchema.parse(result.value)
    expect(evidence.analysis.kind).toBe('validation')
    if(evidence.analysis.kind!=='validation')throw new Error('Wrong diagnostic')
    for(const [actual,expected] of [[evidence.analysis.surrogacy,b.validation_surrogacy],[evidence.analysis.comparability,b.validation_comparability]] as const){
      if(actual.kind!=='estimated')throw new Error('Unexpected perfect fit in noisy oracle fixture')
      for(const [i,value] of [actual.estimate,actual.standardError,actual.tStatistic].entries())expect(Math.abs(value-expected[i]!)).toBeLessThanOrEqual(1e-9*(1+Math.abs(expected[i]!)))
    }
  }
})

test('bias bounds match the unchanged R functions through the browser worker',async({page})=>{
  const cases=z.object({cases:z.array(z.object({experimental:matrix,observational:matrix,baseline:matrix,outcome:z.array(z.number()),treatment:z.array(z.number()),tight_surrogacy:z.array(z.number()),tight_comparability:z.array(z.number())}))}).parse(JSON.parse(readFileSync(new URL('../../octopus/rust-causal-transpile/oracle/surrogate-index/fitted-bounds-fixtures.json',import.meta.url),'utf8'))).cases
  await page.goto('/app')
  for(const b of cases){
    const n=b.baseline[0]!.length-1
    const common={experimental:{surrogates:b.experimental.map(r=>r.slice(1,r.length-n)),treatment:b.treatment},observational:{surrogates:b.observational.map(r=>r.slice(1,r.length-n)),outcome:b.outcome},
      adjustment:{kind:'baseline',experimental:b.experimental.map(r=>r.slice(-n)),observational:b.observational.map(r=>r.slice(-n))}}
    for(const [kind,expected] of [['binaryWithoutSurrogacy',b.tight_surrogacy],['binaryWithoutComparability',b.tight_comparability]] as const){
      const request=surrogateDiagnosticRequestSchema.parse({...common,analysis:{kind:'biasBounds',restriction:{kind}}})
      const result=await page.evaluate(async model=>{const client=await import(new URL('/src/analysis/client.ts',location.href).href);return client.runSurrogateDiagnostics(model)},request)
      expect(result.ok,JSON.stringify(result)).toBe(true)
      const e=surrogateDiagnosticEvidenceSchema.parse(result.value)
      if(e.analysis.kind!=='biasBounds')throw new Error('Wrong diagnostic')
      expect(Math.abs(e.analysis.lower-expected[0]!)).toBeLessThanOrEqual(1e-9)
      expect(Math.abs(e.analysis.upper-expected[1]!)).toBeLessThanOrEqual(1e-9)
    }
  }
})

test('validation and bias controls retain their results through reload',async({page},info)=>{
  test.setTimeout(90000)
  const b=bases.find(b=>b.propensity_design===null&&b.experimental[0]?.length===2)
  if(b===undefined)throw new Error('The pinned oracle must include an unadjusted one-surrogate design.')
  const csv=['sample,treatment,outcome,surrogate',...b.experimental.map((r,i)=>[1,b.treatment[i],b.experimental_outcome[i],r[1]].join(',')),...b.observational.map((r,i)=>[0,'',b.outcome[i],r[1]].join(','))].join('\n')
  const file={name:'surrogate-validation.csv',mimeType:'text/csv',buffer:Buffer.from(csv)}
  await page.goto('/app')
  await page.getByRole('textbox',{name:'Project name',exact:true}).fill('Surrogate diagnostics acceptance')
  await page.getByRole('button',{name:'Create project',exact:true}).click()
  await page.locator('input[type=file]').setInputFiles(file)
  await page.getByRole('button',{name:/Inspect data/}).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({timeout:60000})
  await prepare(page,{structure:'cross-section',columns:['surrogate']})
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{name:'Two-sample surrogates',exact:true}).check()
  await choose(page,'Sample membership','sample')
  await choose(page,'Treatment in experimental sample','treatment')
  await choose(page,'Long-term outcome in observational sample','outcome')
  await page.getByRole('checkbox',{name:'surrogate',exact:true}).first().check()
  await page.getByRole('textbox',{name:'Identifying-assumption rationale'}).fill('Synthetic oracle validation exercise. The outcome is observed in both samples; the regression diagnostics assess the selected linear restrictions, not the full causal assumptions.')
  await page.getByRole('radio',{name:'Outcome observed in both samples',exact:true}).check()
  await choose(page,'Bias-bound restriction','Bounded direct effect')
  await page.getByRole('textbox',{name:'Maximum deviation (outcome units)',exact:true}).fill('0.1')
  await page.getByRole('button',{name:'Estimate long-term effect',exact:true}).click()
  const validation=page.getByRole('region',{name:'Surrogate validation regressions'})
  await expect(validation).toBeVisible({timeout:30000})
  await expect(validation.getByRole('row',{name:/Treatment \(surrogacy\)/})).toContainText(formatStatistic('raw',b.validation_surrogacy[0]!).text)
  const bounds=page.getByTestId('surrogate-estimate').filter({visible:true}).first()
  await expect(bounds).toContainText('Bias bounds')
  await expect(page.getByRole('button',{name:'About Bias bounds'}).first()).toBeVisible()
  await bounds.scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('surrogate-diagnostics.png'),animations:'disabled'})
  await chapter(page,/Results/)
  await expect(validation).toBeVisible()
  await page.reload()
  await page.getByRole('button',{name:'Open Surrogate diagnostics acceptance',exact:true}).click()
  await expect(page.getByRole('heading',{name:'Choose the data file again',exact:true})).toBeVisible()
  await page.locator('input[type=file]').setInputFiles(file)
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{name:'Two-sample surrogates',exact:true}).check()
  await expect(validation).toBeVisible()
  await expect(page.getByRole('radio',{name:'Outcome observed in both samples',exact:true})).toBeChecked()
  await expect(page.getByRole('textbox',{name:'Maximum deviation (outcome units)',exact:true})).toHaveValue('0.1')
})

test('perfect-fit validation is labelled and restored without t inference',async({page},info)=>{
  test.setTimeout(90000)
  const b=bases.find(b=>b.propensity_design===null&&b.experimental[0]?.length===2)
  if(b===undefined)throw new Error('The pinned oracle must include an unadjusted one-surrogate design.')
  const csv=['sample,treatment,outcome,surrogate',...b.experimental.map((r,i)=>[1,b.treatment[i],r[1],r[1]].join(',')),...b.observational.map((r,i)=>[0,'',r[1],r[1]].join(','))].join('\n')
  const file={name:'surrogate-validation.csv',mimeType:'text/csv',buffer:Buffer.from(csv)}
  await page.goto('/app')
  await page.getByRole('textbox',{name:'Project name',exact:true}).fill('Perfect-fit validation acceptance')
  await page.getByRole('button',{name:'Create project',exact:true}).click()
  await page.locator('input[type=file]').setInputFiles(file)
  await page.getByRole('button',{name:/Inspect data/}).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({timeout:60000})
  await prepare(page,{structure:'cross-section',columns:['surrogate']})
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{name:'Two-sample surrogates',exact:true}).check()
  await choose(page,'Sample membership','sample')
  await choose(page,'Treatment in experimental sample','treatment')
  await choose(page,'Long-term outcome in observational sample','outcome')
  await page.getByRole('checkbox',{name:'surrogate',exact:true}).first().check()
  await page.getByRole('textbox',{name:'Identifying-assumption rationale'}).fill('Synthetic oracle validation exercise. The outcome is observed in both samples; the regression diagnostics assess the selected linear restrictions, not the full causal assumptions.')
  await page.getByRole('radio',{name:'Outcome observed in both samples',exact:true}).check()
  await choose(page,'Bias-bound restriction','Bounded direct effect')
  await page.getByRole('textbox',{name:'Maximum deviation (outcome units)',exact:true}).fill('0.1')
  await page.getByRole('button',{name:'Estimate long-term effect',exact:true}).click()
  const validation=page.getByRole('region',{name:'Surrogate validation regressions'})
  await expect(validation).toBeVisible({timeout:30000})
  await expect(validation).toContainText('Validation is uninformative for a numerically perfect fit')
  await expect(validation.getByRole('cell',{name:'Not applicable',exact:true})).toHaveCount(4)
  const bounds=page.getByTestId('surrogate-estimate').filter({visible:true}).first()
  await expect(bounds).toContainText('Bias bounds')
  await expect(page.getByRole('button',{name:'About Bias bounds'}).first()).toBeVisible()
  await bounds.scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('surrogate-diagnostics.png'),animations:'disabled'})
  await chapter(page,/Results/)
  await expect(validation).toBeVisible()
  await page.reload()
  await page.getByRole('button',{name:'Open Perfect-fit validation acceptance',exact:true}).click()
  await expect(page.getByRole('heading',{name:'Choose the data file again',exact:true})).toBeVisible()
  await page.locator('input[type=file]').setInputFiles(file)
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{name:'Two-sample surrogates',exact:true}).check()
  await expect(validation).toBeVisible()
  await expect(page.getByRole('radio',{name:'Outcome observed in both samples',exact:true})).toBeChecked()
  await expect(page.getByRole('textbox',{name:'Maximum deviation (outcome units)',exact:true})).toHaveValue('0.1')
})

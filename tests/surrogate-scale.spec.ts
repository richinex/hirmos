import {test,expect} from '@playwright/test'
import {chapter,choose,prepare} from './examples/support'

test('36 windows are set from one ordered list and retain perfect-fit validation',async({page},info)=>{
  test.setTimeout(120000)
  let seed=73
  const random=()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/4294967296}
  const columns=Array.from({length:36},(_,i)=>'q'+(i+1))
  const lines=['site,treatment,outcome,'+columns.join(',')]
  for(let i=0;i<192;i++){
    const values=columns.map(()=>random())
    lines.push([i<96?'Riverside':i%2===0?'Alameda':'Los Angeles',i<96?i%2:'',values.reduce((a,b)=>a+b,0)/36,...values].join(','))
  }
  const file={name:'36-periods.csv',mimeType:'text/csv',buffer:Buffer.from(lines.join('\n'))}
  await page.goto('/app')
  await page.getByRole('textbox',{name:'Project name',exact:true}).fill('36-period surrogate acceptance')
  await page.getByRole('button',{name:'Create project',exact:true}).click()
  await page.locator('input[type=file]').setInputFiles(file)
  await page.getByRole('button',{name:/Inspect data/}).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({timeout:60000})
  await prepare(page,{structure:'cross-section',columns:['q1']})
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{name:'Two-sample surrogates',exact:true}).check()
  await choose(page,'Sample membership','site')
  await page.getByRole('group',{name:'Experimental sample categories',exact:true}).getByRole('checkbox',{name:'Riverside',exact:true}).check()
  for(const site of ['Alameda','Los Angeles'])await page.getByRole('group',{name:'Observational sample categories',exact:true}).getByRole('checkbox',{name:site,exact:true}).check()
  await choose(page,'Treatment in experimental sample','treatment')
  await choose(page,'Long-term outcome in observational sample','outcome')
  await page.getByRole('button',{name:'Select all short-term surrogates',exact:true}).click()
  await page.getByRole('textbox',{name:'Identifying-assumption rationale'}).fill('Simulated numerical acceptance data. The outcome is the mean of all 36 surrogates, so validation at the full horizon is uninformative.')
  await page.getByRole('radio',{name:'Outcome observed in both samples',exact:true}).check()
  await page.getByRole('radio',{name:'Observed outcomes by period',exact:true}).check()
  const periods=page.getByRole('group',{name:'Outcome periods',exact:true})
  for(const column of columns)await periods.getByRole('checkbox',{name:column,exact:true}).check()
  await expect(page.getByRole('textbox',{name:'Period 36 label',exact:true})).toHaveValue('q36')
  await page.getByRole('radio',{name:'Compare surrogate windows',exact:true}).check()
  await page.getByRole('button',{name:'Mark all window ends',exact:true}).click()
  const order=page.getByRole('list',{name:'Surrogate window order'})
  await expect(order.getByRole('checkbox')).toHaveCount(36)
  await expect(page.getByRole('textbox',{name:'Window label for q36',exact:true})).toHaveValue('q36')
  await order.scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('36-window-order.png')})
  await page.getByRole('button',{name:'Estimate long-term effect',exact:true}).click()
  const validation=page.getByRole('region',{name:'Surrogate validation regressions'})
  await expect(validation).toContainText('numerically perfect fit',{timeout:30000})
  await expect(validation.getByRole('cell',{name:'Not applicable',exact:true})).toHaveCount(4)
  await validation.scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('perfect-fit-validation.png')})
  await expect(page.getByRole('region',{name:'Surrogate window estimates'})).toBeVisible()
})

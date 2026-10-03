import {useEffect,useState} from 'react'
import type {ColumnId,DatasetProfile} from '@/domain/dataset'
import {ColumnChecklist} from '@/components/ui/ColumnChecklist'
import {field,fieldHint,fieldRow} from '@/components/ui/recipes'

type Categories={readonly kind:'loading'}|{readonly kind:'failed';readonly message:string}|{readonly kind:'ready';readonly values:readonly string[]}
export function SurrogateCategoryPicker({file,profile,column,experimental,observational,onChange}:{
  readonly file:File;readonly profile:DatasetProfile;readonly column:ColumnId
  readonly experimental:readonly string[];readonly observational:readonly string[]
  readonly onChange:(experimental:readonly string[],observational:readonly string[])=>void
}){
  const [categories,setCategories]=useState<Categories>({kind:'loading'})
  const [search,setSearch]=useState('')
  useEffect(()=>{
    let current=true
    void import('@/data/client').then(async({summarizeColumnsInWorker})=>{
      const result=await summarizeColumnsInWorker(file,profile,column)
      if(!current)return
      if(!result.ok){setCategories({kind:'failed',message:'The source categories could not be read. Check the selected file and membership column.'});return}
      const values=result.value.columns.find(c=>c.column===column)?.categories
      setCategories(values===null||values===undefined?{kind:'failed',message:'No category values are available for this membership column.'}:{kind:'ready',values:values.map(c=>c.value)})
    }).catch(()=>{if(current)setCategories({kind:'failed',message:'The source categories could not be read.'})})
    return ()=>{current=false}
  },[file,profile,column])
  if(categories.kind==='loading')return <p className={fieldHint} role="status">Reading sample categories…</p>
  if(categories.kind==='failed')return <p className={fieldHint} role="alert">{categories.message}</p>
  const visible=categories.values.filter(v=>v.toLocaleLowerCase().includes(search.toLocaleLowerCase()))
  const choices=visible.map(value=>({id:value,name:value}))
  const preserveHidden=(previous:readonly string[],next:readonly string[])=>[...previous.filter(v=>!visible.includes(v)),...next.filter(v=>visible.includes(v))]
  return <div className="space-y-3">
    {categories.values.length>8&&<div className={fieldRow.two}><input aria-label="Search sample categories" placeholder="Search categories" className={field('text')} value={search} onChange={e=>setSearch(e.target.value)}/></div>}
    <div className={fieldRow.two}>
      <div className="max-h-64 overflow-auto"><ColumnChecklist title="Experimental sample categories" help="Choose the values belonging to the experimental sample. Each value can belong to only one sample. Unselected values are excluded and counted. Missing membership values are not assigned to either sample." columns={choices} selected={experimental} reserved={observational} onChange={values=>onChange(preserveHidden(experimental,values),observational)}/></div>
      <div className="max-h-64 overflow-auto"><ColumnChecklist title="Observational sample categories" help="Choose the values belonging to the observational sample." columns={choices} selected={observational} reserved={experimental} onChange={values=>onChange(experimental,preserveHidden(observational,values))}/></div>
    </div>
  </div>
}

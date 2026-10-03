import type {ReactNode} from 'react'
import type {ColumnId} from '@/domain/dataset'
import type {SurrogateHorizons} from '@/domain/surrogateHorizons'
import {Icon} from '@/components/Icon'
import {SegmentedControl} from '@/components/ui/SegmentedControl'
import {ColumnChecklist} from '@/components/ui/ColumnChecklist'
import {ParameterHelp,ParameterLabel} from '@/components/ui/ParameterLabel'
import {button,field,fieldLabel,iconControl} from '@/components/ui/recipes'
import {cn} from '@/lib/utils'

type Column={readonly id:ColumnId;readonly name:string}
const periodColumns='grid-cols-[1.5rem_minmax(0,1fr)_4.5rem] @md/panel:grid-cols-[2rem_minmax(0,1fr)_minmax(0,1.5fr)_4.5rem]'
const windowColumns='grid-cols-[1.5rem_minmax(0,1fr)_2rem_4.5rem] @md/panel:grid-cols-[2rem_minmax(0,1fr)_6rem_minmax(0,1.5fr)_4.5rem]'
// On a narrow panel the label takes its own line under the row instead of squeezing the name.
const labelCell='order-last col-span-full @md/panel:order-none @md/panel:col-span-1'

/** Periods are kept in their chosen order. Windows are cumulative, so an ordered list and the
 *  surrogates that end a window describe them completely. */
export interface SurrogateHorizonDraft {
  readonly observed:'none'|'periods'
  readonly periods:readonly {readonly column:ColumnId;readonly label:string}[]
  readonly windows:'none'|'groups'
  readonly order:readonly ColumnId[]
  readonly ends:Readonly<Record<string,string>>
}
export const emptySurrogateHorizonDraft=():SurrogateHorizonDraft=>({observed:'none',periods:[],windows:'none',order:[],ends:{}})

export function restoreSurrogateHorizons(h:SurrogateHorizons):SurrogateHorizonDraft{
  const groups=h.windows.kind==='none'?[]:h.windows.groups
  return {observed:h.observed.kind,periods:h.observed.kind==='none'?[]:h.observed.periods,windows:h.windows.kind,
    order:groups.flatMap(g=>g.columns),ends:Object.fromEntries(groups.map(g=>[g.columns.at(-1)!,g.label]))}
}

/** The chosen order for the current surrogates; newly selected surrogates follow in source order. */
function windowOrder(d:SurrogateHorizonDraft,surrogates:readonly Column[]):readonly Column[]{
  const kept=d.order.flatMap(id=>surrogates.filter(c=>c.id===id))
  return [...kept,...surrogates.filter(c=>!d.order.includes(c.id))]
}

function windowGroups(d:SurrogateHorizonDraft,surrogates:readonly Column[]){
  const order=windowOrder(d,surrogates),groups:{label:string;columns:ColumnId[]}[]=[]
  let pending:ColumnId[]=[]
  order.forEach((c,i)=>{
    pending.push(c.id)
    const last=i===order.length-1
    if(c.id in d.ends||last){groups.push({label:d.ends[c.id]??c.name,columns:pending});pending=[]}
  })
  return groups
}

export function requestedSurrogateHorizons(d:SurrogateHorizonDraft,surrogates:readonly Column[]){
  return {observed:d.observed==='none'?{kind:'none'}:{kind:'periods',periods:d.periods},
    windows:d.windows==='none'?{kind:'none'}:{kind:'groups',groups:windowGroups(d,surrogates)}}
}

/** The prompt for an unfinished horizon draft, or null when every row is complete. */
export function surrogateHorizonDraftProblem(d:SurrogateHorizonDraft,surrogates:readonly Column[]):string|null{
  if(d.observed==='periods'&&d.periods.length===0)return 'Choose at least one observed outcome period.'
  if(d.observed==='periods'&&d.periods.some(p=>p.label.trim()===''))return 'Give every observed outcome period a label.'
  if(d.windows==='groups'&&windowGroups(d,surrogates).some(g=>g.label.trim()===''))return 'Give every surrogate window a label.'
  return null
}

function move<T>(values:readonly T[],index:number,offset:number):readonly T[]{
  const next=[...values],other=index+offset
  if(other<0||other>=next.length)return values
  ;[next[index],next[other]]=[next[other]!,next[index]!]
  return next
}

function MoveButtons({name,index,count,onMove}:{readonly name:string;readonly index:number;readonly count:number;readonly onMove:(offset:number)=>void}){
  return <span className="flex justify-end gap-1">
    <button type="button" className={iconControl()} disabled={index===0} aria-label={`Move ${name} earlier`} title="Move earlier" onClick={()=>onMove(-1)}><Icon name="arrow_upward" size={16}/></button>
    <button type="button" className={iconControl()} disabled={index===count-1} aria-label={`Move ${name} later`} title="Move later" onClick={()=>onMove(1)}><Icon name="arrow_downward" size={16}/></button>
  </span>
}

/** A plain ordered list: no frame or row rules, labels editable in place, so it reads as a list rather than a grid. */
function OrderedList({label,title,help,children}:{readonly label:string;readonly title:string;readonly help:string;readonly children:ReactNode}){
  return <div className="max-w-3xl">
    <ParameterLabel className={fieldLabel} label={title} help={help}/>
    <ol className="m-0 mt-1 max-h-96 list-none overflow-auto p-0" aria-label={label}>{children}</ol>
  </div>
}
const row=(columns:string)=>cn('grid items-center gap-3 rounded-md px-2 py-0.5 transition-colors hover:bg-well',columns)
const inlineField=field('text','border-transparent! bg-transparent! py-1 hover:border-control! focus:border-signal/60! focus:bg-well!')

export function SurrogateHorizonControls({draft,onChange,columns,surrogates,reserved}:{
  readonly draft:SurrogateHorizonDraft;readonly onChange:(value:SurrogateHorizonDraft)=>void
  readonly columns:readonly Column[];readonly surrogates:readonly ColumnId[];readonly reserved:readonly ColumnId[]
}){
  const set=(part:Partial<SurrogateHorizonDraft>)=>onChange({...draft,...part})
  const selected=columns.filter(c=>surrogates.includes(c.id))
  const order=windowOrder(draft,selected)
  const name=(id:ColumnId)=>columns.find(c=>c.id===id)?.name??id
  const choosePeriods=(ids:readonly ColumnId[])=>set({periods:[
    ...draft.periods.filter(p=>ids.includes(p.column)),
    ...columns.filter(c=>ids.includes(c.id)&&!draft.periods.some(p=>p.column===c.id)).map(c=>({column:c.id,label:c.name})),
  ]})
  const setEnd=(c:Column,on:boolean)=>{const ends={...draft.ends};if(on)ends[c.id]=c.name;else delete ends[c.id];set({ends})}
  return <div className="min-w-0 space-y-6">
    <div className="space-y-3">
      <div className="flex items-center gap-1.5">
        <SegmentedControl ariaLabel="Observed outcome path" value={draft.observed} onChange={observed=>set({observed})} options={[{value:'none',label:'No observed path'},{value:'periods',label:'Observed outcomes by period'}]}/>
        <ParameterHelp label="Observed outcome path" help="Choose measurements of the same outcome on the same scale, from earliest to latest. Every experimental participant must have all selected measurements. The cumulative mean gives each selected period equal weight."/>
      </div>
      {draft.observed==='periods'?<>
        <ColumnChecklist title="Outcome periods" help="Choose the measurements to plot. They are added in source-column order." columns={columns} selected={draft.periods.map(p=>p.column)} reserved={reserved} onChange={choosePeriods}/>
        {draft.periods.length>0?<OrderedList label="Observed outcome period order" title="Period order and labels" help="Periods run from earliest to latest. Edit a label in place, and use the arrows to reorder.">{draft.periods.map((p,i)=><li key={p.column} className={row(periodColumns)}>
          <span className="text-body text-faint tabular-nums">{i+1}</span>
          <span className="truncate text-body text-ink">{name(p.column)}</span>
          <input aria-label={`Period ${i+1} label`} className={cn(inlineField,labelCell)} value={p.label} onChange={e=>set({periods:draft.periods.map((v,j)=>j===i?{...v,label:e.target.value}:v)})}/>
          <MoveButtons name={`period ${i+1}`} index={i} count={draft.periods.length} onMove={offset=>set({periods:move(draft.periods,i,offset)})}/>
        </li>)}</OrderedList>:null}
      </>:null}
    </div>
    <div className="space-y-3">
      <div className="flex items-center gap-1.5">
        <SegmentedControl ariaLabel="Surrogate window path" value={draft.windows} onChange={windows=>set({windows})} options={[{value:'none',label:'One surrogate window'},{value:'groups',label:'Compare surrogate windows'}]}/>
        <ParameterHelp label="Surrogate window path" help="Order the selected surrogates by when they become available, from earliest to latest, and mark where each window ends. Each window includes every surrogate up to its end. The long-term outcome, participants, baseline covariates and estimator stay fixed."/>
      </div>
      {draft.windows==='groups'?(order.length===0?<p className="m-0 text-body text-muted">Choose short-term surrogates in step 2.</p>:<>
        <div className="flex flex-wrap gap-2">
          <button type="button" className={button('quiet',undefined,'sm')} onClick={()=>set({order:order.map(c=>c.id),ends:Object.fromEntries(order.map(c=>[c.id,draft.ends[c.id]??c.name]))})}>Mark all window ends</button>
          <button type="button" className={button('quiet',undefined,'sm')} onClick={()=>set({order:order.map(c=>c.id),ends:{}})}>Clear window ends</button>
        </div>
        <OrderedList label="Surrogate window order" title="Window ends and labels" help="Surrogates run from earliest to latest. Tick each surrogate that ends a window; the final surrogate always ends one. Each window's label names it in the results.">{order.map((c,i)=>{
          const last=i===order.length-1,ends=last||c.id in draft.ends
          return <li key={c.id} className={row(windowColumns)}>
            <span className="text-body text-faint tabular-nums">{i+1}</span>
            <span className="truncate text-body text-ink">{c.name}</span>
            <span><input type="checkbox" aria-label={`Window ends at ${c.name}`} checked={ends} disabled={last} title={last?'The final surrogate always ends a window.':undefined} onChange={e=>setEnd(c,e.target.checked)}/></span>
            {ends?<input aria-label={`Window label for ${c.name}`} className={cn(inlineField,labelCell)} value={draft.ends[c.id]??c.name} onChange={e=>set({ends:{...draft.ends,[c.id]:e.target.value}})}/>:<span className={labelCell}/>}
            <MoveButtons name={c.name} index={i} count={order.length} onMove={offset=>set({order:move(order.map(o=>o.id),i,offset)})}/>
          </li>
        })}</OrderedList>
      </>):null}
    </div>
  </div>
}

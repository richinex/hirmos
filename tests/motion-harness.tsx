import {useState} from 'react'
import {createRoot} from 'react-dom/client'
import {ReactFlow,ReactFlowProvider} from '@xyflow/react'
import {FlowControls} from '../src/components/flow/FlowControls'
import {SegmentedControl} from '../src/components/ui/SegmentedControl'
import {Orb} from '../src/components/ui/Orb'
import '@xyflow/react/dist/style.css'

function Harness(){
  const [value,setValue]=useState('first')
  return <div className="bg-panel text-ink p-4" style={{position:'fixed',inset:0,zIndex:99999}}>
    <div style={{height:320}}><ReactFlowProvider><ReactFlow nodes={[{id:'a',position:{x:40,y:40},data:{label:'Treatment'}},{id:'b',position:{x:200,y:180},data:{label:'Outcome'}}]} edges={[]} defaultViewport={{x:0,y:0,zoom:1}}><FlowControls fit={{padding:0.2}}/></ReactFlow></ReactFlowProvider></div>
    <SegmentedControl value={value} onChange={setValue} ariaLabel="Motion test choices" options={[{value:'first',label:'First'},{value:'second',label:'Second'}]}/>
    <Orb state="working" aria-label="Analysis running"/>
    <div className="skeleton" style={{height:24,width:180}}/>
    <div className="pulse-live">Working</div>
  </div>
}
export function mount(){const host=document.createElement('div');host.id='motion-harness';document.body.append(host);createRoot(host).render(<Harness/>)}

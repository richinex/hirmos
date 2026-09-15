//! SciPy 1.13.1 `_linesearch.py`: scalar Wolfe-2 search and zoom.
//! Source licence: reference/scipy-1.13.1-LICENSE.txt.

use super::{dot, Counted, LineSearch};

fn cubic(a:f64,fa:f64,fpa:f64,b:f64,fb:f64,c:f64,fc:f64)->Option<f64> {
    let db=b-a;
    let dc=c-a;
    let denominator=(db*dc).powi(2)*(db-dc);
    let v1=fb-fa-fpa*db;
    let v2=fc-fa-fpa*dc;
    let aa=(dc.powi(2)*v1-db.powi(2)*v2)/denominator;
    let bb=(-dc.powi(3)*v1+db.powi(3)*v2)/denominator;
    let value=a+(-bb+(bb*bb-3.0*aa*fpa).sqrt())/(3.0*aa);
    value.is_finite().then_some(value)
}

fn quadratic(a:f64,fa:f64,fpa:f64,b:f64,fb:f64)->Option<f64> {
    let db=b-a;
    let bb=(fb-fa-fpa*db)/(db*db);
    let value=a-fpa/(2.0*bb);
    value.is_finite().then_some(value)
}

struct Search<'a,'b,F,G> {
    ev: &'a mut Counted<'b,F,G>,
    x: &'a [f64],
    direction: &'a [f64],
    phi0: f64,
    derivative0: f64,
    c1:f64,
    c2:f64,
}

impl<F:Fn(&[f64])->f64,G:Fn(&[f64])->Vec<f64>> Search<'_,'_,F,G> {
    fn point(&self,alpha:f64)->Vec<f64> {
        self.x.iter().zip(self.direction).map(|(x,p)|x+alpha*p).collect()
    }
    fn value(&mut self,alpha:f64)->f64 {
        self.ev.fun(&self.point(alpha))
    }
    fn gradient(&mut self,alpha:f64)->(Vec<f64>,f64) {
        let gradient=self.ev.grad(&self.point(alpha));
        let derivative=dot(&gradient,self.direction);
        (gradient,derivative)
    }
    fn accepted(&self,alpha:f64,value:f64,gradient:Vec<f64>)->LineSearch {
        LineSearch {stp:Some(alpha),phi:value,phi0:self.phi0,grad:gradient}
    }
    fn failed(&self)->LineSearch {
        LineSearch {stp:None,phi:self.phi0,phi0:self.phi0,grad:Vec::new()}
    }
    fn zoom(&mut self,mut lo:f64,mut hi:f64,mut flo:f64,mut fhi:f64,mut dlo:f64)->LineSearch {
        let mut recorded_value=self.phi0;
        let mut recorded_step=0.0;
        for iteration in 0..=10 {
            let delta=hi-lo;
            let (a,b)=if delta<0.0 {(hi,lo)} else {(lo,hi)};
            let candidate=if iteration>0 {
                cubic(lo,flo,dlo,hi,fhi,recorded_step,recorded_value)
                    .filter(|&v| v<=b-0.2*delta && v>=a+0.2*delta)
            } else {None};
            let step=candidate.or_else(||quadratic(lo,flo,dlo,hi,fhi)
                .filter(|&v|v<=b-0.1*delta && v>=a+0.1*delta)).unwrap_or(lo+0.5*delta);
            let value=self.value(step);
            if value>self.phi0+self.c1*step*self.derivative0 || value>=flo {
                recorded_value=fhi;
                recorded_step=hi;
                hi=step;
                fhi=value;
                continue;
            }
            let (gradient,derivative)=self.gradient(step);
            if derivative.abs()<=-self.c2*self.derivative0 {
                return self.accepted(step,value,gradient);
            }
            if derivative*(hi-lo)>=0.0 {
                recorded_value=fhi;
                recorded_step=hi;
                hi=lo;
                fhi=flo;
            } else {
                recorded_value=flo;
                recorded_step=lo;
            }
            lo=step;
            flo=value;
            dlo=derivative;
        }
        self.failed()
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn search<F,G>(ev:&mut Counted<F,G>,x:&[f64],direction:&[f64],gradient:&[f64],
    phi0:f64,old_phi0:Option<f64>,c1:f64,c2:f64,amax:f64)->LineSearch
where F:Fn(&[f64])->f64,G:Fn(&[f64])->Vec<f64> {
    let derivative0=dot(gradient,direction);
    let mut alpha0=0.0;
    let mut alpha1=match old_phi0 {
        Some(old) if derivative0!=0.0 => (1.01*2.0*(phi0-old)/derivative0).min(1.0),
        _=>1.0,
    };
    if alpha1<0.0 {alpha1=1.0;}
    alpha1=alpha1.min(amax);
    let mut state=Search {ev,x,direction,phi0,derivative0,c1,c2};
    let mut value1=state.value(alpha1);
    let mut value0=phi0;
    let mut derivative_at0=derivative0;
    for iteration in 0..10 {
        if alpha1==0.0 || alpha0>amax {return state.failed();}
        if value1>phi0+c1*alpha1*derivative0 || (iteration>0 && value1>=value0) {
            return state.zoom(alpha0,alpha1,value0,value1,derivative_at0);
        }
        let (gradient1,derivative1)=state.gradient(alpha1);
        if derivative1.abs()<=-c2*derivative0 {return state.accepted(alpha1,value1,gradient1);}
        if derivative1>=0.0 {return state.zoom(alpha1,alpha0,value1,value0,derivative1);}
        alpha0=alpha1;
        alpha1=(2.0*alpha1).min(amax);
        value0=value1;
        value1=state.value(alpha1);
        derivative_at0=derivative1;
    }
    // SciPy returns the last step without a gradient; BFGS then evaluates it.
    let (gradient,_)=state.gradient(alpha1);
    state.accepted(alpha1,value1,gradient)
}

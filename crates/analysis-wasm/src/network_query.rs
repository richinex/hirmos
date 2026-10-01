use super::*;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NetworkAssignment {
    pub variable: usize,
    pub state: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NetworkQuery {
    pub rows: usize,
    pub columns: usize,
    pub names: Vec<String>,
    pub edges: Vec<(usize, usize)>,
    pub outcomes: Vec<usize>,
    pub interventions: Vec<NetworkAssignment>,
    pub observations: Vec<NetworkAssignment>,
    pub bins: usize,
    pub equivalent_sample_size: f64,
}

pub fn run(values: &[f64], q: NetworkQuery) -> Result<AnalysisResult, String> {
    validate_dense_matrix("network query", values, q.rows, q.columns)?;
    if q.names.len() != q.columns || q.names.iter().any(|n| n.trim().is_empty()) || q.names.iter().collect::<std::collections::BTreeSet<_>>().len() != q.columns {
        return Err("Each network column needs a distinct nonempty name.".into());
    }
    if !(2..=10).contains(&q.bins) || !q.equivalent_sample_size.is_finite() || q.equivalent_sample_size < 0.0 {
        return Err("Use a state budget from 2 to 10 and a nonnegative equivalent sample size.".into());
    }
    if q.edges.iter().any(|&(a,b)| a >= q.columns || b >= q.columns || a == b) {
        return Err("Network edges must connect distinct existing nodes.".into());
    }
    let mut degree = vec![0usize; q.columns];
    for &(_,b) in &q.edges { degree[b] += 1; }
    let mut todo = (0..q.columns).filter(|&i| degree[i] == 0).collect::<Vec<_>>();
    let mut seen = 0;
    while let Some(a) = todo.pop() {
        seen += 1;
        for &(from,b) in &q.edges { if from == a { degree[b] -= 1; if degree[b] == 0 { todo.push(b); } } }
    }
    if seen != q.columns { return Err("The network must be acyclic.".into()); }
    let roles = q.outcomes.iter().copied().chain(q.interventions.iter().map(|a| a.variable)).chain(q.observations.iter().map(|a| a.variable)).collect::<Vec<_>>();
    if q.outcomes.is_empty() || roles.iter().any(|&i| i >= q.columns) || roles.iter().collect::<std::collections::BTreeSet<_>>().len() != roles.len() {
        return Err("Choose at least one outcome and use each variable in only one query role.".into());
    }
    let budget = StateBudget::try_from(q.bins).map_err(|_| "Invalid state budget.")?;
    let mut frame = HashMap::new();
    let mut states = Vec::new();
    for (i,name) in q.names.iter().enumerate() {
        let series = &values[i*q.rows..(i+1)*q.rows];
        let result = discretize_for_discrete_bn(series, budget).map_err(|e| format!("Cannot prepare states for {name}: {e:?}"))?;
        states.push(result.means.into_iter().collect::<Vec<_>>());
        frame.insert(name.clone(), result.labels);
    }
    let resolve = |assignments: &[NetworkAssignment]| -> Result<HashMap<String,String>,String> {
        assignments.iter().map(|a| {
            let state = states[a.variable].get(a.state).ok_or_else(|| format!("State {} is absent for {}.", a.state, q.names[a.variable]))?;
            Ok((q.names[a.variable].clone(), state.0.clone()))
        }).collect()
    };
    let interventions = resolve(&q.interventions)?;
    let observations = resolve(&q.observations)?;
    let edges = q.edges.iter().map(|&(a,b)| (q.names[a].clone(),q.names[b].clone())).collect::<Vec<_>>();
    let bn = DiscreteBn::fit(DiscreteDag::new(&edges,&q.names), &frame, q.equivalent_sample_size);
    let outcomes = q.outcomes.iter().map(|&i| q.names[i].clone()).collect::<Vec<_>>();
    let distribution = bn.distribution(&outcomes, &interventions, &observations)?;
    Ok(AnalysisResult::NetworkQuery { observations: q.rows, states, distribution })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn network_query_matches_pinned_bnlearn() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../causal-core/oracle/fixtures/network_query_bnlearn.json")).unwrap();
        let patterns=fixture["patterns"].as_array().unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let counts=case["counts"].as_array().unwrap();
            let rows=counts.iter().map(|c|c.as_u64().unwrap() as usize).sum::<usize>();
            let mut values=Vec::new();
            for col in 0..4 { for (p,c) in patterns.iter().zip(counts) { values.extend(std::iter::repeat_n(p[col].as_f64().unwrap(),c.as_u64().unwrap() as usize)); } }
            for expected in case["queries"].as_array().unwrap() {
                let q:NetworkQuery=serde_json::from_value(serde_json::json!({"rows":rows,"columns":4,"names":fixture["names"],"edges":fixture["edges"],"outcomes":expected["outcomes"],"observations":expected["observations"],"interventions":expected["interventions"],"bins":3,"equivalentSampleSize":case["equivalentSampleSize"]})).unwrap();
                let result=run(&values,q).unwrap();
                let AnalysisResult::NetworkQuery{distribution,..}=result else {panic!("wrong result")};
                let reference:Vec<(Vec<String>,f64)>=serde_json::from_value(expected["distribution"].clone()).unwrap();
                assert_eq!(distribution.len(),reference.len());
                for ((states,p),(rstates,rp)) in distribution.iter().zip(reference) {assert_eq!(*states,rstates);assert!((p-rp).abs()<1e-12,"{} {}: {p} != {rp}",case["name"],expected["name"]);}
            }
        }
    }
    #[test]
    fn network_query_rejects_invalid_roles_states_and_graphs() {
        let base=serde_json::json!({"rows":4,"columns":2,"names":["X","Y"],"edges":[[0,1]],"outcomes":[1],"observations":[],"interventions":[{"variable":0,"state":1}],"bins":3,"equivalentSampleSize":0});
        let values=[0.,0.,1.,1.,0.,1.,0.,1.];
        for (field,value) in [("observations",serde_json::json!([{"variable":0,"state":0}])),("interventions",serde_json::json!([{"variable":0,"state":2}])),("outcomes",serde_json::json!([])),("edges",serde_json::json!([[0,1],[1,0]]))] {
            let mut q=base.clone();q[field]=value;
            assert!(run(&values,serde_json::from_value(q).unwrap()).is_err());
        }
    }
}

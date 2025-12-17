// SPICE parser support for controlled sources
// This module handles parsing of controlled sources from SPICE netlists

use pest::iterators::Pair;
use crate::frontends::spice::Rule;
use crate::{Variable, Element, FrontendError};
use crate::models::Unit;
use std::sync::Arc;
use std::collections::HashMap;
use crate::spot::Numeric;
use super::{VCVSBundle, VCCSBundle, CCCSBundle, CCVSBundle};
use crate::models::controlled_sources::vcvrs::VCVSOptions;
use crate::models::controlled_sources::vccs::VCCSOptions;
use crate::models::controlled_sources::cccs::CCCSOptions;
use crate::models::controlled_sources::ccvs::CCVSOptions;

/// Processes a VCVS (E) source from SPICE format
/// Syntax: E<name> <pos> <neg> <ctrl_pos> <ctrl_neg> <gain>
pub fn process_vcvs(
    element: Pair<Rule>,
    variables: &mut Vec<Variable>,
    elements: &mut Vec<Element>,
    var_map: &mut std::collections::HashMap<Arc<str>, usize>,
) -> Result<(), FrontendError> {
    let mut inner = element.into_inner();
    
    // Parse name
    let name = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing name for VCVS source".into()))?;
    let name_str = name.as_str();
    
    // Parse nodes
    let pos_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing positive node for VCVS source".into()))?;
    let neg_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing negative node for VCVS source".into()))?;
    let ctrl_pos_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing controlling positive node for VCVS source".into()))?;
    let ctrl_neg_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing controlling negative node for VCVS source".into()))?;
    
    // Parse gain
    let gain_value = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing gain value for VCVS source".into()))?;
    let gain: Numeric = gain_value.as_str().parse().map_err(|_| 
        FrontendError::ParseError("Invalid gain value for VCVS source".into()))?;
    
    // Create variables and add to var_map
    let pos_var = Variable::new(Arc::from(pos_node.as_str()), Unit::Volt, variables.len());
    let neg_var = Variable::new(Arc::from(neg_node.as_str()), Unit::Volt, variables.len() + 1);
    let ctrl_pos_var = Variable::new(Arc::from(ctrl_pos_node.as_str()), Unit::Volt, variables.len() + 2);
    let ctrl_neg_var = Variable::new(Arc::from(ctrl_neg_node.as_str()), Unit::Volt, variables.len() + 3);
    
    // Add variables to vectors and map
    variables.push(pos_var.clone());
    var_map.insert(Arc::from(pos_node.as_str()), variables.len() - 1);
    
    variables.push(neg_var.clone());
    var_map.insert(Arc::from(neg_node.as_str()), variables.len() - 1);
    
    variables.push(ctrl_pos_var.clone());
    var_map.insert(Arc::from(ctrl_pos_node.as_str()), variables.len() - 1);
    
    variables.push(ctrl_neg_var.clone());
    var_map.insert(Arc::from(ctrl_neg_node.as_str()), variables.len() - 1);
    
    // Create VCVS bundle
    let vcvs = VCVSBundle::new(
        Arc::from(name_str),
        Some(pos_var),
        Some(neg_var),
        Some(ctrl_pos_var),
        Some(ctrl_neg_var),
        Some(VCVSOptions { gain }),
    );
    
    elements.push(Element::VCVS(vcvs));
    
    Ok(())
}

/// Processes a VCCS (G) source from SPICE format
/// Syntax: G<name> <pos> <neg> <ctrl_pos> <ctrl_neg> <transconductance>
pub fn process_vccs(
    element: Pair<Rule>,
    variables: &mut Vec<Variable>,
    elements: &mut Vec<Element>,
    var_map: &mut HashMap<Arc<str>, usize>,
) -> Result<(), FrontendError> {
    let mut inner = element.into_inner();
    
    // Parse name
    let name = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing name for VCCS source".into()))?;
    let name_str = name.as_str();
    
    // Parse nodes
    let pos_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing positive node for VCCS source".into()))?;
    let neg_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing negative node for VCCS source".into()))?;
    let ctrl_pos_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing controlling positive node for VCCS source".into()))?;
    let ctrl_neg_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing controlling negative node for VCCS source".into()))?;
    
    // Parse transconductance
    let trans_value = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing transconductance value for VCCS source".into()))?;
    let transconductance: Numeric = trans_value.as_str().parse().map_err(|_| 
        FrontendError::ParseError("Invalid transconductance value for VCCS source".into()))?;
    
    // Create variables and add to var_map
    let pos_var = Variable::new(Arc::from(pos_node.as_str()), Unit::Volt, variables.len());
    let neg_var = Variable::new(Arc::from(neg_node.as_str()), Unit::Volt, variables.len() + 1);
    let ctrl_pos_var = Variable::new(Arc::from(ctrl_pos_node.as_str()), Unit::Volt, variables.len() + 2);
    let ctrl_neg_var = Variable::new(Arc::from(ctrl_neg_node.as_str()), Unit::Volt, variables.len() + 3);
    
    // Add variables to vectors and map
    variables.push(pos_var.clone());
    var_map.insert(Arc::from(pos_node.as_str()), variables.len() - 1);
    
    variables.push(neg_var.clone());
    var_map.insert(Arc::from(neg_node.as_str()), variables.len() - 1);
    
    variables.push(ctrl_pos_var.clone());
    var_map.insert(Arc::from(ctrl_pos_node.as_str()), variables.len() - 1);
    
    variables.push(ctrl_neg_var.clone());
    var_map.insert(Arc::from(ctrl_neg_node.as_str()), variables.len() - 1);
    
    // Create VCCS bundle
    let vccs = VCCSBundle::new(
        Arc::from(name_str),
        Some(pos_var),
        Some(neg_var),
        Some(ctrl_pos_var),
        Some(ctrl_neg_var),
        Some(VCCSOptions { transconductance }),
    );
    
    elements.push(Element::VCCS(vccs));
    
    Ok(())
}

/// Processes a CCCS (F) source from SPICE format
/// Syntax: F<name> <pos> <neg> <ctrl_branch> <gain>
pub fn process_cccs(
    element: Pair<Rule>,
    variables: &mut Vec<Variable>,
    elements: &mut Vec<Element>,
    var_map: &mut HashMap<Arc<str>, usize>,
) -> Result<(), FrontendError> {
    let mut inner = element.into_inner();
    
    // Parse name
    let name = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing name for CCCS source".into()))?;
    let name_str = name.as_str();
    
    // Parse nodes
    let pos_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing positive node for CCCS source".into()))?;
    let neg_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing negative node for CCCS source".into()))?;
    let ctrl_branch_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing controlling branch node for CCCS source".into()))?;
    
    // Parse gain
    let gain_value = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing gain value for CCCS source".into()))?;
    let gain: Numeric = gain_value.as_str().parse().map_err(|_| 
        FrontendError::ParseError("Invalid gain value for CCCS source".into()))?;
    
    // Create variables and add to var_map
    let pos_var = Variable::new(Arc::from(pos_node.as_str()), Unit::Volt, variables.len());
    let neg_var = Variable::new(Arc::from(neg_node.as_str()), Unit::Volt, variables.len() + 1);
    let ctrl_branch_var = Variable::new(Arc::from(ctrl_branch_node.as_str()), Unit::Volt, variables.len() + 2);
    
    // Add variables to vectors and map
    variables.push(pos_var.clone());
    var_map.insert(Arc::from(pos_node.as_str()), variables.len() - 1);
    
    variables.push(neg_var.clone());
    var_map.insert(Arc::from(neg_node.as_str()), variables.len() - 1);
    
    variables.push(ctrl_branch_var.clone());
    var_map.insert(Arc::from(ctrl_branch_node.as_str()), variables.len() - 1);
    
    // Create CCCS bundle
    let cccs = CCCSBundle::new(
        Arc::from(name_str),
        Some(pos_var),
        Some(neg_var),
        Some(ctrl_branch_var),
        Some(CCCSOptions { gain }),
    );
    
    elements.push(Element::CCCS(cccs));
    
    Ok(())
}

/// Processes a CCVS (H) source from SPICE format
/// Syntax: H<name> <pos> <neg> <ctrl_branch> <gain>
pub fn process_ccvs(
    element: Pair<Rule>,
    variables: &mut Vec<Variable>,
    elements: &mut Vec<Element>,
    var_map: &mut HashMap<Arc<str>, usize>,
) -> Result<(), FrontendError> {
    let mut inner = element.into_inner();
    
    // Parse name
    let name = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing name for CCVS source".into()))?;
    let name_str = name.as_str();
    
    // Parse nodes
    let pos_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing positive node for CCVS source".into()))?;
    let neg_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing negative node for CCVS source".into()))?;
    let ctrl_branch_node = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing controlling branch node for CCVS source".into()))?;
    
    // Parse gain
    let gain_value = inner.next().ok_or_else(|| 
        FrontendError::ParseError("Missing gain value for CCVS source".into()))?;
    let gain: Numeric = gain_value.as_str().parse().map_err(|_| 
        FrontendError::ParseError("Invalid gain value for CCVS source".into()))?;
    
    // Create variables and add to var_map
    let pos_var = Variable::new(Arc::from(pos_node.as_str()), Unit::Volt, variables.len());
    let neg_var = Variable::new(Arc::from(neg_node.as_str()), Unit::Volt, variables.len() + 1);
    let ctrl_branch_var = Variable::new(Arc::from(ctrl_branch_node.as_str()), Unit::Volt, variables.len() + 2);
    
    // Add variables to vectors and map
    variables.push(pos_var.clone());
    var_map.insert(Arc::from(pos_node.as_str()), variables.len() - 1);
    
    variables.push(neg_var.clone());
    var_map.insert(Arc::from(neg_node.as_str()), variables.len() - 1);
    
    variables.push(ctrl_branch_var.clone());
    var_map.insert(Arc::from(ctrl_branch_node.as_str()), variables.len() - 1);
    
    // Create CCVS bundle
    let ccvs = CCVSBundle::new(
        Arc::from(name_str),
        Some(pos_var),
        Some(neg_var),
        Some(ctrl_branch_var),
        Some(CCVSOptions { gain }),
    );
    
    elements.push(Element::CCVS(ccvs));
    
    Ok(())
}
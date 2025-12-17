// Serde support for controlled sources
// This module will handle serialization/deserialization of controlled sources
// from/to JSON/YAML formats

use std::sync::Arc;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::{Variable, Element};
use crate::models::Unit;
use crate::frontends::serde::ProcessSerdeElement;
use super::{VCVSBundle, VCCSBundle, CCCSBundle, CCVSBundle};
use crate::models::controlled_sources::vcvrs::VCVSOptions;
use crate::models::controlled_sources::vccs::VCCSOptions;
use crate::models::controlled_sources::cccs::CCCSOptions;
use crate::models::controlled_sources::ccvs::CCVSOptions;


/// Serde representation of a VCVS source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeVCVS {
    pub name: String,
    pub positive: String,
    pub negative: String,
    pub controlling_positive: String,
    pub controlling_negative: String,
    pub gain: f64,
}

/// Serde representation of a VCCS source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeVCCS {
    pub name: String,
    pub positive: String,
    pub negative: String,
    pub controlling_positive: String,
    pub controlling_negative: String,
    pub transconductance: f64,
}

/// Serde representation of a CCCS source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeCCCS {
    pub name: String,
    pub positive: String,
    pub negative: String,
    pub controlling_branch: String,
    pub gain: f64,
}

/// Serde representation of a CCVS source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeCCVS {
    pub name: String,
    pub positive: String,
    pub negative: String,
    pub controlling_branch: String,
    pub gain: f64,
}

impl ProcessSerdeElement for SerdeVCVS {
    fn process(
        &self,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        // Create variables for output nodes
        let pos_var = Variable::new(Arc::from(self.positive.as_str()), Unit::Volt, variables.len());
        let neg_var = Variable::new(Arc::from(self.negative.as_str()), Unit::Volt, variables.len() + 1);
        
        // Add variables to vectors and map
        variables.push(pos_var.clone());
        var_map.insert(Arc::from(self.positive.as_str()), variables.len() - 1);
        
        variables.push(neg_var.clone());
        var_map.insert(Arc::from(self.negative.as_str()), variables.len() - 1);
        
        // Create variables for controlling nodes
        let ctrl_pos_var = Variable::new(Arc::from(self.controlling_positive.as_str()), Unit::Volt, variables.len());
        let ctrl_neg_var = Variable::new(Arc::from(self.controlling_negative.as_str()), Unit::Volt, variables.len() + 1);
        
        // Add controlling variables to vectors and map
        variables.push(ctrl_pos_var.clone());
        var_map.insert(Arc::from(self.controlling_positive.as_str()), variables.len() - 1);
        
        variables.push(ctrl_neg_var.clone());
        var_map.insert(Arc::from(self.controlling_negative.as_str()), variables.len() - 1);
        
        // Create VCVS bundle
        let vcvs = VCVSBundle::new(
            Arc::from(self.name.as_str()),
            Some(pos_var),
            Some(neg_var),
            Some(ctrl_pos_var),
            Some(ctrl_neg_var),
            Some(VCVSOptions { gain: self.gain }),
        );
        
        elements.push(Element::VCVS(vcvs));
    }
}

impl ProcessSerdeElement for SerdeVCCS {
    fn process(
        &self,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        // Create variables for output nodes
        let pos_var = Variable::new(Arc::from(self.positive.as_str()), Unit::Volt, variables.len());
        let neg_var = Variable::new(Arc::from(self.negative.as_str()), Unit::Volt, variables.len() + 1);
        
        // Add variables to vectors and map
        variables.push(pos_var.clone());
        var_map.insert(Arc::from(self.positive.as_str()), variables.len() - 1);
        
        variables.push(neg_var.clone());
        var_map.insert(Arc::from(self.negative.as_str()), variables.len() - 1);
        
        // Create variables for controlling nodes
        let ctrl_pos_var = Variable::new(Arc::from(self.controlling_positive.as_str()), Unit::Volt, variables.len());
        let ctrl_neg_var = Variable::new(Arc::from(self.controlling_negative.as_str()), Unit::Volt, variables.len() + 1);
        
        // Add controlling variables to vectors and map
        variables.push(ctrl_pos_var.clone());
        var_map.insert(Arc::from(self.controlling_positive.as_str()), variables.len() - 1);
        
        variables.push(ctrl_neg_var.clone());
        var_map.insert(Arc::from(self.controlling_negative.as_str()), variables.len() - 1);
        
        // Create VCCS bundle
        let vccs = VCCSBundle::new(
            Arc::from(self.name.as_str()),
            Some(pos_var),
            Some(neg_var),
            Some(ctrl_pos_var),
            Some(ctrl_neg_var),
            Some(VCCSOptions { transconductance: self.transconductance }),
        );
        
        elements.push(Element::VCCS(vccs));
    }
}

impl ProcessSerdeElement for SerdeCCCS {
    fn process(
        &self,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        // Create variables for output nodes
        let pos_var = Variable::new(Arc::from(self.positive.as_str()), Unit::Volt, variables.len());
        let neg_var = Variable::new(Arc::from(self.negative.as_str()), Unit::Volt, variables.len() + 1);
        
        // Add variables to vectors and map
        variables.push(pos_var.clone());
        var_map.insert(Arc::from(self.positive.as_str()), variables.len() - 1);
        
        variables.push(neg_var.clone());
        var_map.insert(Arc::from(self.negative.as_str()), variables.len() - 1);
        
        // Create variable for controlling branch
        let ctrl_branch_var = Variable::new(Arc::from(self.controlling_branch.as_str()), Unit::Volt, variables.len());
        
        // Add controlling variable to vectors and map
        variables.push(ctrl_branch_var.clone());
        var_map.insert(Arc::from(self.controlling_branch.as_str()), variables.len() - 1);
        
        // Create CCCS bundle
        let cccs = CCCSBundle::new(
            Arc::from(self.name.as_str()),
            Some(pos_var),
            Some(neg_var),
            Some(ctrl_branch_var),
            Some(CCCSOptions { gain: self.gain }),
        );
        
        elements.push(Element::CCCS(cccs));
    }
}

impl ProcessSerdeElement for SerdeCCVS {
    fn process(
        &self,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        // Create variables for output nodes
        let pos_var = Variable::new(Arc::from(self.positive.as_str()), Unit::Volt, variables.len());
        let neg_var = Variable::new(Arc::from(self.negative.as_str()), Unit::Volt, variables.len() + 1);
        
        // Add variables to vectors and map
        variables.push(pos_var.clone());
        var_map.insert(Arc::from(self.positive.as_str()), variables.len() - 1);
        
        variables.push(neg_var.clone());
        var_map.insert(Arc::from(self.negative.as_str()), variables.len() - 1);
        
        // Create variable for controlling branch
        let ctrl_branch_var = Variable::new(Arc::from(self.controlling_branch.as_str()), Unit::Volt, variables.len());
        
        // Add controlling variable to vectors and map
        variables.push(ctrl_branch_var.clone());
        var_map.insert(Arc::from(self.controlling_branch.as_str()), variables.len() - 1);
        
        // Create CCVS bundle
        let ccvs = CCVSBundle::new(
            Arc::from(self.name.as_str()),
            Some(pos_var),
            Some(neg_var),
            Some(ctrl_branch_var),
            Some(CCVSOptions { gain: self.gain }),
        );
        
        elements.push(Element::CCVS(ccvs));
    }
}
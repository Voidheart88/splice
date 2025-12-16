//! Helper functions and structures for SPICE element parsing
//! This module provides abstractions to reduce code duplication in element parsing

use std::sync::Arc;
use pest::iterators::Pair;
use crate::frontends::FrontendError;
use crate::spot::Numeric;

/// Helper structure for parsing SPICE elements
/// Provides common parsing functionality to reduce code duplication
pub struct SpiceElementParser<'a> {
    ele: &'a str,
    offset: usize,
    inner: std::iter::Peekable<pest::iterators::Pairs<'a, crate::frontends::spice::Rule>>,
}

impl<'a> SpiceElementParser<'a> {
    /// Create a new parser for a SPICE element
    pub fn new(element: Pair<'a, crate::frontends::spice::Rule>) -> Self {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let inner = element.into_inner().peekable();
        
        SpiceElementParser {
            ele,
            offset,
            inner,
        }
    }
    
    /// Parse the element name
    pub fn parse_name(&mut self, element_type: &str) -> Result<&'a str, FrontendError> {
        let name_end = self.inner.next()
            .ok_or_else(|| FrontendError::ParseError(
                format!("Missing name in {}: {}", element_type, self.ele)
            ))?
            .as_span().end() - self.offset;
        Ok(&self.ele[0..name_end])
    }
    
    /// Parse a node reference
    pub fn parse_node(&mut self, element_type: &str, element_name: &str, node_name: &str) -> Result<&'a str, FrontendError> {
        let node_span = self.inner.next()
            .ok_or_else(|| FrontendError::ParseError(
                format!("Missing {} in {} '{}'", node_name, element_type, element_name)
            ))?
            .as_span();
        Ok(&self.ele[node_span.start() - self.offset..node_span.end() - self.offset])
    }
    
    /// Parse a numeric value
    pub fn parse_value(&mut self, element_type: &str, element_name: &str, value_name: &str) -> Result<Numeric, FrontendError> {
        let value_span = self.inner.next()
            .ok_or_else(|| FrontendError::ParseError(
                format!("Missing {} in {} '{}'", value_name, element_type, element_name)
            ))?
            .as_span();
        
        let value_str = &self.ele[value_span.start() - self.offset..value_span.end() - self.offset];
        value_str.parse::<Numeric>()
            .map_err(|_| FrontendError::ParseError(
                format!("Invalid {} in {} '{}': must be a number", value_name, element_type, element_name)
            ))
    }
    
    /// Parse an optional numeric value
    pub fn parse_optional_value(&mut self) -> Option<Result<Numeric, FrontendError>> {
        self.inner.next().map(|pair| {
            let value_span = pair.as_span();
            let value_str = &self.ele[value_span.start() - self.offset..value_span.end() - self.offset];
            value_str.parse::<Numeric>()
                .map_err(|_| FrontendError::ParseError(
                    format!("Invalid optional value: must be a number")
                ))
        })
    }
    
    /// Parse remaining values as strings
    pub fn parse_remaining_values(&mut self) -> Vec<&'a str> {
        let mut result = Vec::new();
        while let Some(pair) = self.inner.next() {
            let span = pair.as_span();
            result.push(&self.ele[span.start() - self.offset..span.end() - self.offset]);
        }
        result
    }
    
    /// Check if there are more values to parse
    pub fn has_more(&mut self) -> bool {
        self.inner.peek().is_some()
    }
}

/// Helper function to create a standard error message for missing elements
pub fn missing_element_error(element_type: &str, ele: &str) -> FrontendError {
    FrontendError::ParseError(format!("Missing name in {}: {}", element_type, ele))
}

/// Helper function to create a standard error message for missing nodes
pub fn missing_node_error(element_type: &str, element_name: &str, node_name: &str) -> FrontendError {
    FrontendError::ParseError(format!("Missing {} in {} '{}'", node_name, element_type, element_name))
}

/// Helper function to create a standard error message for invalid values
pub fn invalid_value_error(element_type: &str, element_name: &str, value_name: &str) -> FrontendError {
    FrontendError::ParseError(format!("Invalid {} in {} '{}': must be a number", value_name, element_type, element_name))
}

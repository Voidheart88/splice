//! Helper functions and structures for SPICE element parsing
//! This module provides abstractions to reduce code duplication in element parsing

use crate::frontends::FrontendError;
use crate::spot::Numeric;
use pest::iterators::Pair;

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

        SpiceElementParser { ele, offset, inner }
    }

    /// Helper to slice the underlying element string from a span
    fn span_str(&self, span: pest::Span<'a>) -> &'a str {
        &self.ele[span.start() - self.offset..span.end() - self.offset]
    }

    /// Parse the element name
    pub fn parse_name(&mut self, element_type: &str) -> Result<&'a str, FrontendError> {
        let pair = self
            .inner
            .next()
            .ok_or_else(|| missing_element_error(element_type, self.ele))?;
        let name_end = pair.as_span().end() - self.offset;
        Ok(&self.ele[0..name_end])
    }

    /// Parse a node reference
    pub fn parse_node(
        &mut self,
        element_type: &str,
        element_name: &str,
        node_name: &str,
    ) -> Result<&'a str, FrontendError> {
        let pair = self
            .inner
            .next()
            .ok_or_else(|| missing_node_error(element_type, element_name, node_name))?;
        Ok(self.span_str(pair.as_span()))
    }

    /// Parse a numeric value
    pub fn parse_value(
        &mut self,
        element_type: &str,
        element_name: &str,
        value_name: &str,
    ) -> Result<Numeric, FrontendError> {
        let pair = self.inner.next().ok_or_else(|| {
            FrontendError::ParseError(format!(
                "Missing {} in {} '{}'",
                value_name, element_type, element_name
            ))
        })?;

        self.span_str(pair.as_span())
            .parse::<Numeric>()
            .map_err(|_| invalid_value_error(element_type, element_name, value_name))
    }

    /// Parse a numeric value with a default if not present
    pub fn parse_value_or_default(
        &mut self,
        _element_type: &str,
        _element_name: &str,
        _value_name: &str,
        default: Numeric,
    ) -> Result<Numeric, FrontendError> {
        // Try to parse the value, return default if not available
        if let Some(pair) = self.inner.next() {
            self.span_str(pair.as_span())
                .parse::<Numeric>()
                .map_err(|_| {
                    FrontendError::ParseError(format!("Invalid {} - must be a number", _value_name))
                })
        } else {
            Ok(default)
        }
    }

    /// Parse an optional numeric value
    #[allow(dead_code)]
    pub fn parse_optional_value(&mut self) -> Option<Result<Numeric, FrontendError>> {
        self.inner.next().map(|pair| {
            self.span_str(pair.as_span())
                .parse::<Numeric>()
                .map_err(|_| {
                    FrontendError::ParseError(
                        "Invalid optional value: must be a number".to_string(),
                    )
                })
        })
    }

    /// Parse remaining values as strings
    pub fn parse_remaining_values(&mut self) -> Vec<&'a str> {
        let ele = self.ele;
        let offset = self.offset;
        self.inner
            .by_ref()
            .map(|pair| {
                let span = pair.as_span();
                &ele[span.start() - offset..span.end() - offset]
            })
            .collect()
    }

    /// Check if there are more values to parse
    #[allow(dead_code)]
    pub fn has_more(&mut self) -> bool {
        self.inner.peek().is_some()
    }
}

/// Helper function to create a standard error message for missing elements
pub fn missing_element_error(element_type: &str, ele: &str) -> FrontendError {
    FrontendError::ParseError(format!("Missing name in {}: {}", element_type, ele))
}

/// Helper function to create a standard error message for missing nodes
pub fn missing_node_error(
    element_type: &str,
    element_name: &str,
    node_name: &str,
) -> FrontendError {
    FrontendError::ParseError(format!(
        "Missing {} in {} '{}'",
        node_name, element_type, element_name
    ))
}

/// Helper function to create a standard error message for invalid values
pub fn invalid_value_error(
    element_type: &str,
    element_name: &str,
    value_name: &str,
) -> FrontendError {
    FrontendError::ParseError(format!(
        "Invalid {} in {} '{}': must be a number",
        value_name, element_type, element_name
    ))
}

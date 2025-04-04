use std::marker::PhantomData;

use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};

/// Describes the operations available on a single path.
/// A Path Item MAY be empty, due to ACL constraints.
/// The path itself is still exposed to the documentation
/// viewer but they will not know which operations and
/// parameters are available.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PathItem {
  /// An optional, string summary, intended to apply to all operations in
  /// this path.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub summary: Option<String>,
  /// An optional, string description, intended to apply to all operations in
  /// this path. CommonMark syntax MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// A definition of a GET operation on this path.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub get: Option<Operation>,
  /// A definition of a PUT operation on this path.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub put: Option<Operation>,
  /// A definition of a POST operation on this path.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub post: Option<Operation>,
  /// A definition of a DELETE operation on this path.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub delete: Option<Operation>,
  /// A definition of a OPTIONS operation on this path.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub options: Option<Operation>,
  /// A definition of a HEAD operation on this path.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub head: Option<Operation>,
  /// A definition of a PATCH operation on this path.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub patch: Option<Operation>,
  /// A definition of a TRACE operation on this path.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub trace: Option<Operation>,
  /// An alternative server array to service all operations in this path.
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub servers: Vec<Server>,
  /// A list of parameters that are applicable for all the
  /// operations described under this path. These parameters
  /// can be overridden at the operation level, but cannot be
  /// removed there. The list MUST NOT include duplicated parameters.
  /// A unique parameter is defined by a combination of a name and location.
  /// The list can use the Reference Object to link to parameters that
  /// are defined at the OpenAPI Object's components/parameters.
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub parameters: Vec<ReferenceOr<Parameter>>,
  /// Inline extensions to this object.
  #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
  pub extensions: IndexMap<String, serde_json::Value>,
}

impl PathItem {
  /// Returns an iterator of references to the [Operation]s in the [PathItem].
  pub fn iter(&self) -> impl Iterator<Item = (&str, &'_ Operation)> {
    vec![
      ("get", &self.get),
      ("put", &self.put),
      ("post", &self.post),
      ("delete", &self.delete),
      ("options", &self.options),
      ("head", &self.head),
      ("patch", &self.patch),
      ("trace", &self.trace),
    ]
    .into_iter()
    .filter_map(|(method, maybe_op)| maybe_op.as_ref().map(|op| (method, op)))
  }

  /// Returns an iterator of mutable references to the [Operation]s in the [PathItem].
  pub fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &'_ mut Operation)> {
    vec![
      ("get", &mut self.get),
      ("put", &mut self.put),
      ("post", &mut self.post),
      ("delete", &mut self.delete),
      ("options", &mut self.options),
      ("head", &mut self.head),
      ("patch", &mut self.patch),
      ("trace", &mut self.trace),
    ]
    .into_iter()
    .filter_map(|(method, maybe_op)| maybe_op.as_mut().map(|op| (method, op)))
  }
}

impl IntoIterator for PathItem {
  type Item = (&'static str, Operation);

  type IntoIter = std::vec::IntoIter<Self::Item>;

  /// Returns an iterator of the [Operation]s in the [PathItem].
  fn into_iter(self) -> Self::IntoIter {
    vec![
      ("get", self.get),
      ("put", self.put),
      ("post", self.post),
      ("delete", self.delete),
      ("options", self.options),
      ("head", self.head),
      ("patch", self.patch),
      ("trace", self.trace),
    ]
    .into_iter()
    .filter_map(|(method, maybe_op)| maybe_op.map(|op| (method, op)))
    .collect::<Vec<_>>()
    .into_iter()
  }
}

/// Holds the relative paths to the individual endpoints and
/// their operations. The path is appended to the URL from the
/// Server Object in order to construct the full URL. The Paths
/// MAY be empty, due to ACL constraints.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Paths {
  /// A map of PathItems or references to them.
  #[serde(flatten, deserialize_with = "deserialize_paths")]
  pub paths: IndexMap<String, ReferenceOr<PathItem>>,
  /// Inline extensions to this object.
  #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
  pub extensions: IndexMap<String, serde_json::Value>,
}

impl Paths {
  /// Iterate over path items.
  pub fn iter(&self) -> indexmap::map::Iter<String, ReferenceOr<PathItem>> {
    self.paths.iter()
  }
}

impl IntoIterator for Paths {
  type Item = (String, ReferenceOr<PathItem>);

  type IntoIter = indexmap::map::IntoIter<String, ReferenceOr<PathItem>>;

  fn into_iter(self) -> Self::IntoIter {
    self.paths.into_iter()
  }
}

fn deserialize_paths<'de, D>(
  deserializer: D,
) -> Result<IndexMap<String, ReferenceOr<PathItem>>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_map(PredicateVisitor(
    |key: &String| key.starts_with('/'),
    PhantomData,
  ))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_path_item_iterators() {
    let operation = Operation::default();

    let path_item = PathItem {
      get: Some(operation.clone()),
      post: Some(operation.clone()),
      delete: Some(operation.clone()),
      ..Default::default()
    };

    let expected = vec![
      ("get", &operation),
      ("post", &operation),
      ("delete", &operation),
    ];
    assert_eq!(path_item.iter().collect::<Vec<_>>(), expected);

    let expected = vec![
      ("get", operation.clone()),
      ("post", operation.clone()),
      ("delete", operation.clone()),
    ];
    assert_eq!(path_item.into_iter().collect::<Vec<_>>(), expected);
  }
}

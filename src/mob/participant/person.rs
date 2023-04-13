use maud::{Markup, Render};
use schema::Schema;
use serde::{Deserialize, Serialize};

use crate::url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Schema)]
/// The public details about a person
pub(crate) struct Person {
    /// The person's name
    ///
    /// Example:
    ///
    /// ```yaml
    /// Nompomer Pilento
    /// ```
    pub(crate) name: PersonName,
    /// A social URL
    ///
    /// Example:
    ///
    /// ```yaml
    /// https://example.com/np
    /// ```
    pub(crate) social_url: Url,
    /// An avatar image URL
    ///
    /// Example:
    ///
    /// ```yaml
    /// https://example.com/np.png
    /// ```
    pub(crate) avatar_url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct PersonName(String);

impl Render for PersonName {
    fn render(&self) -> Markup {
        self.0.render()
    }
}

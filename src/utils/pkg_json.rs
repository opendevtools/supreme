// Simplified version of npm-package-json
// https://github.com/SirWindfield/npm-package-json

use crate::utils::helpers;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::ErrorKind;
use std::{collections::BTreeMap, fs};

/// An ordered map for `bin` entries.
pub type BinSet = BTreeMap<String, String>;
/// An ordered map for `dependencies` entries.
pub type DepsSet = BTreeMap<String, String>;
/// An ordered map for `engines` entries.
pub type EnginesSet = BTreeMap<String, String>;
/// An ordered map for `scripts` entries.
pub type ScriptsSet = BTreeMap<String, String>;

/// The result type of this crate.
pub type Result<T> = std::result::Result<T, std::io::Error>;

/// A bug contacting form.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Bug {
    /// The email to use for contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The url to use to submit bugs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// A person.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Person {
    /// The name of a person.
    pub name: String,
    /// The email of a person.
    pub email: Option<String>,
    /// The homepage of the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// A reference to a person.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PersonReference {
    /// A short reference.
    ///
    /// Short references have a fixed format of `John Doe <john@doe.dev> (https://john.doe.dev)`.
    Short(String),
    /// A full reference.
    ///
    /// This type of reference defines the parts using a struct instead of a
    /// shorthand string format.
    Full(Person),
}

/// A reference to a man page.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ManReference {
    /// A single man page reference. Points to one single file.
    Single(String),
    /// Multiple man pages, can contain anything from zero to n.
    Multiple(Vec<String>),
}

/// A repository.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Repository {
    /// The version control system that the repository uses.
    pub r#type: String,
    /// The url to the repository.
    pub url: String,
    /// The directory that the repository is in. Often used for monorepos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
}

/// A repository reference.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RepositoryReference {
    /// A short reference to the repository. Has to have the syntax that `npm install` allows as well. For more information see [here](https://docs.npmjs.com/files/package.json#repository).
    Short(String),
    /// A full reference.
    Full(Repository),
}

/// The top-level `package.json` structure.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    /// The package name.
    pub name: String,
    /// The package version.
    pub version: String,
    /// The optional package description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The optional package main entry file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<String>,
    /// The optional directories
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub directories: BTreeMap<String, String>,
    /// The optional list of script entries.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub scripts: ScriptsSet,
    /// The optional repository reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RepositoryReference>,
    /// The optional list of keywords.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    /// The optional author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<PersonReference>,
    /// The optional package license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// The optional bug contact form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bugs: Option<Bug>,
    /// The optional package homepage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    /// The optional list of contributors.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributors: Vec<PersonReference>,
    /// The optional list of files to include. Each entry defines a regex
    /// pattern.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    /// The optional package browser entry file.
    ///
    /// This is usually defined in libraries that are meant to be consumed by
    /// browsers. These Thoes can refer to objects that are not available inside
    /// a `nodejs` environment (like `window`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    /// The optional set of binary definitions.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub bin: BinSet,
    /// The optional list of man page references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub man: Option<ManReference>,
    /// The optional list of dependencies.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub dependencies: DepsSet,
    /// The optional list of development dependencies.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub dev_dependencies: DepsSet,
    /// The optional list of peer dependencies.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub peer_dependencies: DepsSet,
    /// The optional list of bundled dependencies.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub bundled_dependencies: DepsSet,
    /// The optional list of optional dependencies.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub optional_dependencies: DepsSet,
    /// The optional list of engine entries.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub engines: EnginesSet,
    /// The package privacy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    /// The OS' that the package can run on.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub os: Vec<String>,
    /// The CPU architectures that the package can run on.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cpu: Vec<String>,
    /// The optional config object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
    /// Other custom fields that have been defined inside the `package.json`
    /// file.
    #[serde(flatten)]
    pub others: BTreeMap<String, Value>,
}

impl Package {
    pub fn new() -> Result<Self> {
        match fs::metadata("package.json") {
            Ok(_) => (),
            Err(err) => {
                // Initialize an empty package.json if none exists
                if let ErrorKind::NotFound = err.kind() {
                    helpers::run_command("npm", &["init", "-y"]);
                }
            }
        };

        let content = fs::read_to_string("package.json")?;
        Ok(serde_json::from_str(&content)?)
    }
}

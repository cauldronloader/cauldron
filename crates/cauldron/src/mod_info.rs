use std::ffi::{CStr, CString, c_char};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CauldronModInfo {
    /// Name of the mod.
    ///
    /// Used for matching against [dependencies](cauldron::CauldronModDependency).
    ///
    /// Required, cannot be null.
    pub name: *const c_char,
    /// [Semver](https://semver.org/spec/v2.0.0.html) version of the mod.
    ///
    /// It's recommended to use the `CARGO_PKG_VERSION` environment variable or equivalent in your build environment.
    ///
    /// Required, cannot be null.
    pub version: *const c_char,
    /// Human-readable variant of [`name`](cauldron::CauldronModInfo.name).
    ///
    /// Optional, may be null.
    pub display_name: *const c_char,
    /// Description of the mod.
    ///
    /// Optional, may be null.
    pub description: *const c_char,
    /// Link to the mod's homepage.
    ///
    /// Optional, may be null.
    pub homepage_url: *const c_char,
    /// Link to the mod's source code.
    ///
    /// Optional, may be null.
    pub source_url: *const c_char,
    /// Link to the mod's issue tracker.
    ///
    /// Optional, may be null.
    pub issue_tracker_url: *const c_char,
    /// Length of the [`authors`](cauldron::CauldronModInfo.authors) array.
    pub authors_len: u32,
    /// Mod authors array.
    ///
    /// An optional email address may be included within angled brackets at the end of each author entry.
    ///
    /// Optional, may be null.
    pub authors: *const *const c_char,
    /// Length of the [`authors`](cauldron::CauldronModInfo.authors) array.
    pub depends_len: u32,
    /// Mod [dependencies](cauldron::CauldronModDependency).
    ///
    /// The loader will ensure mods listed here will be loaded before the current mod.
    ///
    /// Optional, may be null.
    pub depends: *const CauldronModDependency,
}

/// A [CauldronModInfo]'s [dependency](cauldron::CauldronModInfo.depends).
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CauldronModDependency {
    /// Matched against a mod's [`name`](cauldron::CauldronModInfo.name) field.
    ///
    /// Required, cannot be null.
    pub name: *const c_char,
    /// Semver constraint.
    ///
    /// This is matched using [`semver::VersionReq`](https://docs.rs/semver/latest/semver/struct.VersionReq.html).
    ///
    /// Optional, may be null.
    pub version: *const c_char,
    /// Being optional won't cause the loader to error out when it isn't present,
    /// it'll just ensure this mod is loaded after the dependency.
    ///
    /// If an optional mod is present but doesn't match the [`version`](cauldron::CauldronModDependency.version)
    /// constraint it will still cause the loader to error out.
    ///
    pub optional: bool,
}

pub struct CauldronModInfoBuilder {
    name: String,
    version: String,

    display_name: Option<String>,
    description: Option<String>,
    homepage_url: Option<String>,
    source_url: Option<String>,
    issue_tracker_url: Option<String>,

    authors: Option<Vec<String>>,
    depends: Option<Vec<CauldronModDependency>>,
}

impl CauldronModInfo {
    pub fn builder(name: &'static str, version: &'static str) -> CauldronModInfoBuilder {
        CauldronModInfoBuilder {
            name: name.to_owned(),
            version: version.to_owned(),
            display_name: None,
            description: None,
            homepage_url: None,
            source_url: None,
            issue_tracker_url: None,
            authors: None,
            depends: None,
        }
    }
}

impl CauldronModDependency {
    pub fn new(name: &'static str, version: Option<&'static str>, optional: bool) -> Self {
        let c_name = CString::new(name.to_owned()).unwrap();
        let c_version = version.map(|v| CString::new(v.to_owned()).unwrap());

        CauldronModDependency {
            name: c_name.into_raw(),
            version: match c_version {
                None => std::ptr::null(),
                Some(c_version) => c_version.into_raw(),
            },
            optional,
        }
    }
}

impl CauldronModInfoBuilder {
    pub fn display_name(mut self, display_name: &'static str) -> Self {
        self.display_name = Some(display_name.to_owned());
        self
    }

    pub fn description(mut self, description: &'static str) -> Self {
        self.description = Some(description.to_owned());
        self
    }

    pub fn homepage_url(mut self, homepage_url: &'static str) -> Self {
        self.homepage_url = Some(homepage_url.to_owned());
        self
    }

    pub fn source_url(mut self, source_url: &'static str) -> Self {
        self.source_url = Some(source_url.to_owned());
        self
    }

    pub fn issue_tracker_url(mut self, issue_tracker_url: &'static str) -> Self {
        self.issue_tracker_url = Some(issue_tracker_url.to_owned());
        self
    }

    pub fn authors(mut self, authors: Vec<String>) -> Self {
        self.authors = Some(authors);
        self
    }

    pub fn author(mut self, author: &'static str) -> Self {
        let mut authors = self.authors.unwrap_or_default().clone();
        authors.push(author.to_owned());
        self.authors = Some(authors);
        self
    }

    pub fn dependencies(mut self, dependencies: Vec<CauldronModDependency>) -> Self {
        self.depends = Some(dependencies);
        self
    }

    pub fn dependency(mut self, dependency: CauldronModDependency) -> Self {
        let mut dependencies = self.depends.unwrap_or_default().clone();
        dependencies.push(dependency);
        self.depends = Some(dependencies);
        self
    }

    pub fn build(self) -> CauldronModInfo {
        CauldronModInfo {
            name: CString::new(self.name).unwrap().into_raw() as *const c_char,
            version: CString::new(self.version).unwrap().into_raw(),

            display_name: match self.display_name.map(|s| CString::new(s).unwrap()) {
                None => std::ptr::null(),
                Some(c_display_name) => c_display_name.into_raw(),
            },
            description: match self.description.map(|s| CString::new(s).unwrap()) {
                None => std::ptr::null(),
                Some(c_description) => c_description.into_raw(),
            },
            homepage_url: match self.homepage_url.map(|s| CString::new(s).unwrap()) {
                None => std::ptr::null(),
                Some(c_homepage_url) => c_homepage_url.into_raw(),
            },
            source_url: match self.source_url.map(|s| CString::new(s).unwrap()) {
                None => std::ptr::null(),
                Some(c_source_url) => c_source_url.into_raw(),
            },
            issue_tracker_url: match self.issue_tracker_url.map(|s| CString::new(s).unwrap()) {
                None => std::ptr::null(),
                Some(c_issue_tracker_url) => c_issue_tracker_url.into_raw(),
            },

            authors_len: self.authors.clone().map_or(0, |a| a.len() as u32),
            authors: self.authors.map_or(std::ptr::null(), |authors| {
                let cstr_authors: Vec<_> = authors
                    .iter()
                    .map(|arg| CString::new(arg.as_str()).unwrap())
                    .collect();

                let p_authors: Vec<_> = cstr_authors.iter().map(|arg| arg.as_ptr()).collect();

                let p = p_authors.as_ptr();
                std::mem::forget(cstr_authors);
                std::mem::forget(p_authors);

                p
            }),

            depends_len: self.depends.clone().map_or(0, |d| d.len() as u32),
            depends: self.depends.map_or(std::ptr::null(), |deps| {
                // deps.shrink_to_fit();
                deps.as_ptr()
            }),
        }
    }
}

unsafe impl Send for CauldronModInfo {}
unsafe impl Sync for CauldronModInfo {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SafeCauldronModInfo {
    pub name: String,
    pub version: String,

    pub display_name: Option<String>,
    pub description: Option<String>,
    pub homepage_url: Option<String>,
    pub source_url: Option<String>,
    pub issue_tracker_url: Option<String>,

    pub authors: Vec<String>,
    pub dependencies: Vec<SafeCauldronModDependency>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SafeCauldronModDependency {
    pub name: String,
    pub version: Option<String>,
    pub optional: bool,
}

impl From<CauldronModInfo> for SafeCauldronModInfo {
    fn from(value: CauldronModInfo) -> Self {
        SafeCauldronModInfo {
            name: unsafe { CStr::from_ptr(value.name).to_str().unwrap().to_owned() },
            version: unsafe { CStr::from_ptr(value.version).to_str().unwrap().to_owned() },

            display_name: if value.display_name.is_null() {
                None
            } else {
                Some(unsafe {
                    CStr::from_ptr(value.display_name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            description: if value.description.is_null() {
                None
            } else {
                Some(unsafe {
                    CStr::from_ptr(value.description)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            homepage_url: if value.homepage_url.is_null() {
                None
            } else {
                Some(unsafe {
                    CStr::from_ptr(value.homepage_url)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            source_url: if value.homepage_url.is_null() {
                None
            } else {
                Some(unsafe {
                    CStr::from_ptr(value.source_url)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            issue_tracker_url: if value.issue_tracker_url.is_null() {
                None
            } else {
                Some(unsafe {
                    CStr::from_ptr(value.issue_tracker_url)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },

            authors: if value.authors_len > 0 && !value.authors.is_null() {
                unsafe {
                    std::slice::from_raw_parts(value.authors, value.authors_len as usize)
                        .iter()
                        .map(|author| CStr::from_ptr(*author).to_str().unwrap().to_owned())
                        .collect()
                }
            } else {
                Vec::new()
            },
            dependencies: if value.depends_len > 0 && !value.depends.is_null() {
                unsafe {
                    std::slice::from_raw_parts(value.depends, value.depends_len as usize)
                        .iter()
                        .map(|dep| SafeCauldronModDependency::from(dep))
                        .collect()
                }
            } else {
                Vec::new()
            },
        }
    }
}

impl From<&CauldronModDependency> for SafeCauldronModDependency {
    fn from(value: &CauldronModDependency) -> Self {
        SafeCauldronModDependency {
            name: unsafe { CStr::from_ptr(value.name).to_str().unwrap().to_owned() },
            version: if value.version.is_null() {
                None
            } else {
                Some(unsafe { CStr::from_ptr(value.version).to_str().unwrap().to_owned() })
            },
            optional: value.optional,
        }
    }
}

use core::fmt;
use core::str::FromStr;

use smol_str::SmolStr;

/// The maximum number of characters allowed in a GitHub username.
///
/// Obtained by trying to create a new account with these many characters + 1.
const GITHUB_HANDLE_MAX_CHARS: u8 = 39;

/// TODO: docs.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct GitHubHandle {
    inner: SmolStr,
}

impl GitHubHandle {
    /// TODO: docs.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

impl fmt::Debug for GitHubHandle {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for GitHubHandle {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl FromStr for GitHubHandle {
    type Err = GitHubHandleFromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for c in s.chars() {
            if !c.is_ascii_alphanumeric() && c != '-' {
                return Err(GitHubHandleFromStrError::InvalidCharacter(c));
            }
        }

        if s.len() > GITHUB_HANDLE_MAX_CHARS as usize {
            return Err(GitHubHandleFromStrError::TooLong {
                len: s.len(),
                max: GITHUB_HANDLE_MAX_CHARS,
            });
        }

        if s.starts_with('-') {
            return Err(GitHubHandleFromStrError::BeginsWithHyphen);
        }

        if s.ends_with('-') {
            return Err(GitHubHandleFromStrError::EndsWithHyphen);
        }

        Ok(Self { inner: SmolStr::new(s) })
    }
}

impl TryFrom<&str> for GitHubHandle {
    type Error = GitHubHandleFromStrError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

/// The type of error returned when trying to construct a [`GitHubHandle`] from
/// a string.
#[derive(Debug)]
pub enum GitHubHandleFromStrError {
    /// TODO: docs.
    TooLong {
        /// TODO: docs.
        len: usize,
        /// TODO: docs.
        max: u8,
    },

    /// TODO: docs.
    InvalidCharacter(char),

    /// TODO: docs.
    BeginsWithHyphen,

    /// TODO: docs.
    EndsWithHyphen,
}

impl fmt::Display for GitHubHandleFromStrError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitHubHandleFromStrError::TooLong { len, max } => {
                write!(f, "GitHub handle is too long: {len} > {max}")
            },
            GitHubHandleFromStrError::InvalidCharacter(c) => {
                write!(f, "invalid character in GitHub handle: '{c}'")
            },
            GitHubHandleFromStrError::BeginsWithHyphen => {
                write!(f, "GitHub handle cannot begin with a hyphen")
            },
            GitHubHandleFromStrError::EndsWithHyphen => {
                write!(f, "GitHub handle cannot end with a hyphen")
            },
        }
    }
}

impl AsRef<Self> for GitHubHandle {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

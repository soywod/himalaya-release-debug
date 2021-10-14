//! Mailboxes entity module.
//!
//! This module contains the definition of the mailboxes and its traits implementations.

use serde::Serialize;
use std::{
    fmt::{self, Display},
    ops::Deref,
};

use crate::{
    domain::{Mbox, RawMbox},
    ui::Table,
};

/// Represents a list of raw mailboxes returned by the `imap` crate.
pub(crate) type RawMboxes = imap::types::ZeroCopy<Vec<RawMbox>>;

/// Represents a list of mailboxes.
#[derive(Debug, Default, Serialize)]
pub struct Mboxes<'a>(pub Vec<Mbox<'a>>);

/// Derefs the mailboxes to its inner vector.
impl<'a> Deref for Mboxes<'a> {
    type Target = Vec<Mbox<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Makes the mailboxes displayable.
impl<'a> Display for Mboxes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n{}", Table::render(&self))
    }
}

/// Converts a list of `imap::types::Name` into mailboxes.
impl<'a> From<&'a RawMboxes> for Mboxes<'a> {
    fn from(raw_mboxes: &'a RawMboxes) -> Mboxes<'a> {
        Self(raw_mboxes.iter().map(Mbox::from).collect())
    }
}
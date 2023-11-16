use std::collections::{HashSet, HashMap};

use bc_components::{ARID, PublicKeyBase, PrivateKeyBase};
use bc_envelope::prelude::*;
use bc_ur::UREncodable;
use bytes::Bytes;

use crate::Receipt;


pub fn prefix(s: &str, len: usize) -> String {
    s.chars().take(len).collect()
}

pub trait Prefix {
    fn prefix(&self, len: usize) -> String;
}

impl<T> Prefix for T
where
    T: AsRef<str>,
{
    fn prefix(&self, len: usize) -> String {
        prefix(self.as_ref(), len)
    }
}


pub fn suffix(s: &str, len: usize) -> String {
    s.chars().rev().take(len).collect::<String>().chars().rev().collect()
}

pub trait Suffix {
    fn suffix(&self, len: usize) -> String;
}

impl<T> Suffix for T
where
    T: AsRef<str>,
{
    fn suffix(&self, len: usize) -> String {
        suffix(self.as_ref(), len)
    }
}


pub fn flanked_by(s: &str, left: &str, right: &str) -> String {
    left.to_owned() + s + right
}

pub trait FlankedBy {
    fn flanked_by(&self, left: &str, right: &str) -> String;
}

impl<T> FlankedBy for T
where
    T: AsRef<str>,
{
    fn flanked_by(&self, left: &str, right: &str) -> String {
        flanked_by(self.as_ref(), left, right)
    }
}

pub fn flanked_abbrev(s: &str) -> String {
    s.flanked_by("<", ">")
}

pub trait FlankedAbbrev {
    fn flanked_abbrev(&self) -> String;
}

impl<T> FlankedAbbrev for T
where
    T: AsRef<str>,
{
    fn flanked_abbrev(&self) -> String {
        flanked_abbrev(self.as_ref())
    }
}

pub fn flanked_function(s: &str) -> String {
    s.flanked_by("«", "»")
}

pub trait FlankedFunction {
    fn flanked_function(&self) -> String;
}

impl<T> FlankedFunction for T
where
    T: AsRef<str>,
{
    fn flanked_function(&self) -> String {
        flanked_function(self.as_ref())
    }
}

pub trait Abbrev {
    fn abbrev(&self) -> String;
}

fn abbreviate_string(s: impl AsRef<str>) -> String {
    s.as_ref().prefix(8).flanked_abbrev()
}

fn abbreviate_opt_string(s: Option<impl AsRef<str>>) -> String {
    if let Some(s) = s {
        abbreviate_string(s)
    } else {
        "<None>".to_string()
    }
}

impl Abbrev for str {
    fn abbrev(&self) -> String {
        abbreviate_string(self)
    }
}

impl Abbrev for String {
    fn abbrev(&self) -> String {
        abbreviate_string(self)
    }
}

impl Abbrev for Option<String> {
    fn abbrev(&self) -> String {
        abbreviate_opt_string(self.as_deref())
    }
}

impl Abbrev for Option<&String> {
    fn abbrev(&self) -> String {
        abbreviate_opt_string(self.as_deref())
    }
}

impl Abbrev for Option<&str> {
    fn abbrev(&self) -> String {
        abbreviate_opt_string(self.as_deref())
    }
}

impl Abbrev for ARID {
    fn abbrev(&self) -> String {
        self.ur_string().suffix(8).flanked_abbrev()
    }
}

impl Abbrev for PublicKeyBase {
    fn abbrev(&self) -> String {
        self.ur_string().suffix(8).flanked_abbrev()
    }
}

impl Abbrev for PrivateKeyBase {
    fn abbrev(&self) -> String {
        self.ur_string().suffix(8).flanked_abbrev()
    }
}

impl Abbrev for Envelope {
    fn abbrev(&self) -> String {
        self.ur_string().suffix(8).flanked_abbrev()
    }
}

impl Abbrev for Receipt {
    fn abbrev(&self) -> String {
        self.envelope().ur_string().suffix(8).flanked_abbrev()
    }
}

impl Abbrev for Bytes {
    fn abbrev(&self) -> String {
        format!("{} bytes", self.len()).flanked_abbrev()
    }
}

impl<T> Abbrev for Vec<T>
where
    T: Abbrev,
{
    fn abbrev(&self) -> String {
        let mut items = self.iter().map(|i| i.abbrev()).collect::<Vec<_>>();
        items.sort();
        items.join(", ").flanked_by("[", "]")
    }
}

impl<T> Abbrev for HashSet<T>
where
    T: Abbrev,
{
    fn abbrev(&self) -> String {
        let mut items = self.iter().map(|i| i.abbrev()).collect::<Vec<_>>();
        items.sort();
        items.join(", ").flanked_by("[", "]")
    }
}

impl<K, V> Abbrev for HashMap<K, V>
where
    K: Abbrev,
    V: Abbrev,
{
    fn abbrev(&self) -> String {
        let mut items = self
            .iter()
            .map(|(k, v)| format!("{}: {}", k.abbrev(), v.abbrev()))
            .collect::<Vec<_>>();
        items.sort();
        items.join(", ").flanked_by("{", "}")
    }
}

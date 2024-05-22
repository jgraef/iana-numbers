//! # IANA service names and transport protocol ports
//!
//! This crate parses the service list from IANA([1]) during compilation and
//! exposes a constant with it. Currently only system services (port < 1024) are
//! considered.
//!
//! [1]: https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml

use std::iter::FusedIterator;

mod generated {
    include!(concat!(std::env!("OUT_DIR"), "/generated.rs"));
}

/// Transport protocol
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransportProtocol {
    Udp,
    Tcp,
    Sctp,
    Dccp,
}

/// A network service using a port, with a given name.
#[derive(Clone, Copy, Debug)]
pub struct Service {
    pub name: &'static str,
    pub port: u16,
    pub transport_protocol: Option<TransportProtocol>,
}

/// Static slice of all system services.
pub const SERVICES: &'static [Service] = generated::SERVICES;

/// Iterate over all system services
pub fn iter() -> Iter {
    Iter {
        inner: IterInner::All(SERVICES.iter()),
    }
}

/// Returns an iterator over all system services that use the specified port.
pub fn by_port(port: u16) -> Iter {
    Iter {
        inner: IterInner::Some(
            generated::BY_PORT
                .get(&port)
                .copied()
                .unwrap_or_default()
                .into_iter(),
        ),
    }
}

/// Returns an iterator over all system services by the specified name.
pub fn by_name(name: &str) -> Iter {
    Iter {
        inner: IterInner::Some(
            generated::BY_NAME
                .get(name)
                .copied()
                .unwrap_or_default()
                .into_iter(),
        ),
    }
}

#[derive(Clone, Debug)]
enum IterInner {
    All(std::slice::Iter<'static, Service>),
    Some(std::slice::Iter<'static, &'static Service>),
}

/// Iterator over services
#[derive(Clone, Debug)]
pub struct Iter {
    inner: IterInner,
}

impl Iterator for Iter {
    type Item = Service;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            IterInner::All(iter) => iter.next().copied(),
            IterInner::Some(iter) => iter.next().map(|&&s| s),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.inner {
            IterInner::All(iter) => iter.size_hint(),
            IterInner::Some(iter) => iter.size_hint(),
        }
    }
}

impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            IterInner::All(iter) => iter.next_back().copied(),
            IterInner::Some(iter) => iter.next_back().map(|&&s| s),
        }
    }
}

impl ExactSizeIterator for Iter {}

impl FusedIterator for Iter {}

# IANA service names and transport protocol ports

This crate parses the service list from IANA([1]) during compilation and
exposes a constant with it. Currently only system services (port < 1024) are
considered.

[1]: https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml

use std::{
    collections::HashMap,
    fmt::Write as _,
    hash::Hash,
    io::Write as _,
};

use iana_build_tools::{
    phf::{
        phf::PhfHash,
        phf_codegen::Map,
        phf_shared::FmtConst,
    },
    Error,
};
use serde::Deserialize;

const SERVICE_NAMES_FILE: &'static str = "service-names-port-numbers.csv";

#[derive(Debug, Deserialize)]
struct Record {
    // Service Name
    // Port Number
    // Transport Protocol
    // Description
    // Assignee
    // Contact
    // Registration Date
    // Modification Date
    // Reference
    // Service Code
    // Unauthorized Use Reported
    // Assignment Notes
    #[serde(rename = "Service Name")]
    service_name: String,

    #[serde(rename = "Port Number")]
    port_number: String,

    #[serde(rename = "Transport Protocol")]
    transport_protocol: Option<TransportProtocol>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TransportProtocol {
    Udp,
    Tcp,
    Sctp,
    Dccp,
}

fn main() -> Result<(), Error> {
    let mut writer = iana_build_tools::out_file("generated.rs");
    writeln!(&mut writer, "use crate::{{Service, TransportProtocol}};")?;

    let records = iana_build_tools::parse::<Record>(SERVICE_NAMES_FILE);

    let mut by_port: HashMap<u16, Vec<usize>> = HashMap::new();
    let mut by_name: HashMap<String, Vec<usize>> = HashMap::new();

    writeln!(&mut writer, "pub const SERVICES: &'static [Service] = &[")?;
    let mut i = 0;
    for record in records {
        if record.service_name.is_empty() {
            continue;
        }
        let Ok(port) = record.port_number.parse::<u16>()
        else {
            continue;
        };
        if port >= 1024 {
            // for now we limit it to system ports.
            break;
        }

        writeln!(
            &mut writer,
            r#"
    Service {{
        name: {:?},
        port: {port},
        transport_protocol: {},
    }},
            "#,
            record.service_name,
            record.transport_protocol.map_or_else(
                || "None".to_owned(),
                |tp| format!("Some(TransportProtocol::{tp:?})")
            ),
        )?;

        by_port.entry(port).or_default().push(i);
        by_name
            .entry(record.service_name.clone())
            .or_default()
            .push(i);

        i += 1;
    }
    writeln!(&mut writer, "];")?;

    writeln!(
        &mut writer,
        "pub static BY_PORT: phf::Map<u16, &'static [&'static Service]> = {};",
        convert_map(by_port).build(),
    )?;
    writeln!(
        &mut writer,
        "pub static BY_NAME: phf::Map<&'static str, &'static [&'static Service]> = {};",
        convert_map(by_name).build(),
    )?;

    Ok(())
}

fn convert_map<K: Hash + Eq + PhfHash + FmtConst>(input: HashMap<K, Vec<usize>>) -> Map<K> {
    let mut output = Map::new();

    for (key, indices) in input {
        let mut pointers = String::from("&[");
        for index in &indices {
            write!(&mut pointers, "&SERVICES[{}], ", index).unwrap();
        }
        pointers.push_str("] as &'static [&'static Service]");

        output.entry(key, &pointers);
    }

    output
}

use std::{error::Error, io, path::PathBuf};
use serde_xml_rs;
use serde::{Deserialize, Serialize};
use csv::{Terminator, Writer};

mod util;


pub fn from_file(xml: &PathBuf) -> Result<Scan, serde_xml_rs::Error> {
    
    let file = std::fs::read_to_string(xml)?;

    return from_str(file)
}

pub fn from_str<I: Into<String>>(buffer: I) -> Result<Scan, serde_xml_rs::Error> {
    return serde_xml_rs::from_reader(buffer.into().as_bytes());
}

#[derive(serde::Serialize)]
struct Row {
    host_ip: String,
    item: ReportItem,
}



pub fn as_json<W: io::Write>(list_of_nessus: Vec<PathBuf>, w: W) -> Result<(), Box<dyn Error>> {

    let mut collector: Vec<ReportHost> = vec![];

    for nessus in list_of_nessus {
        let scan = from_file(&nessus)?;

        for r in scan.report.iter() {
            println!("[+] Found: {}", r.name);
            collector.extend_from_slice(&r.report_host);
        }
    }

    // Serialize the collector vector into JSON and write it using the writer provided
    serde_json::to_writer(w, &collector)?;

    Ok(())
}

pub fn as_csv<W: io::Write>(list_of_nessus: Vec<PathBuf>, w: W) -> Result<(), Box<dyn Error>>  {
    
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .terminator(Terminator::CRLF)
        .from_writer(w);

    let header = vec![
        "hostip", "port", "svc_name", "protocol", "severity", "plugin_id", "plugin_name", "plugin_family", "agent",
        "always_run", "description", "fname", "plugin_modification_date", "plugin_publication_date",
        "risk_factor", "script_version", "solution", "synopsis", "thorough_tests", "plugin_output",
        "see_also", "bid", "cve", "xref", "patch_publication_date", "vuln_publication_date",
        "exploitability_ease", "exploit_available", "exploit_framework_canvas", "exploit_framework_metasploit",
        "exploit_framework_core", "metasploit_name", "canvas_package", "cvss_vector", "cvss_base_score",
        "cvss_temporal_score", "change", "plugin_type", "plugin_version", "cm_complianceinfo",
        "cm_complianceresult", "cm_complianceactualvalue", "cm_compliancecheck_id",
    ];
    
    wtr.write_record(header)?;

    for nessus in list_of_nessus {
        let scan = from_file(&nessus)?;
        to(&scan, &mut wtr)?;
    }
    
    wtr.flush()?;
    Ok(())
}

fn to<W: io::Write>(scan: &Scan, wtr: &mut Writer<W>) -> Result<(), Box<dyn Error>> {
    for r in scan.report.iter() {
        println!();
        util::info(format!("[+] Found: {}", r.name));

        for host_report in r.report_host.iter() {
            let host_ip = &host_report.name;
            
            for item in host_report.report_item.iter() {
                
                let row: Row = Row { host_ip: host_ip.to_string(), item: item.clone() };

                if let Err(e) = wtr.serialize(&row) {
                    util::error(format!("[Error] Failed to serialize row {:?}: {}", host_ip, e));
                }
            }
        }
    }

    Ok(())
}


// NessusClientDataV2
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Scan {
    pub value: Option<String>,

    #[serde(rename = "Policy")]
    pub policy: Vec<Policy>,
    #[serde(rename = "Report")]
    pub report: Vec<Report>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Preferences {
    #[serde(rename = "ServerPreferences")]
    pub server_preferences: ServerPreferences,
    #[serde(rename = "PluginsPreferences")]
    pub plugins_preferences: PluginsPreferences,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ServerPreferences {
    #[serde(rename = "preference")]   
    pub preferences: Vec<Preference>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PluginsPreferences {
    #[serde(rename = "item")]
    pub items: Vec<Item>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FamilySelection {
    #[serde(rename = "FamilyItem")]
    pub family_items: Vec<FamilyItem>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FamilyItem {
    #[serde(rename = "FamilyName")]
    pub family_name: String,
    #[serde(rename = "Status")]
    pub status: String
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IndividualPluginSelection {
    #[serde(rename = "PluginItem")]
    pub plugin_items: Vec<PluginItem>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PluginItem {
    #[serde(rename = "PluginId")]
    pub plugin_id: Option<String>,
    #[serde(rename = "PluginName")]
    pub plugin_name: Option<String>,
    #[serde(rename = "Family")]
    pub family: Option<String>,   
    #[serde(rename = "Status")]
    pub status: Option<String>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Policy {
    #[serde(rename = "policyName")]
    pub policy_name: String,
    #[serde(rename = "policyComments")]
    pub policy_comments: Option<String>,
    #[serde(rename = "Preferences")]
    pub preferences: Preferences,
    #[serde(rename = "FamilySelection")]
    pub family_selection: Option<FamilySelection>,
    #[serde(rename = "IndividualPluginSelection")]
    pub individual_plugin_selection: Option<IndividualPluginSelection>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Preference {
    pub name: String,
    pub value: String
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "pluginName")]
    pub plugin_name: String,
    #[serde(rename = "pluginId")]
    pub plugin_id: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "preferenceName")]
    pub preference_name: String,
    #[serde(rename = "preferenceType")]
    pub preference_type: String,
    #[serde(rename = "preferenceValues")]
    pub preference_values: String,
    #[serde(rename = "selectedValue")]
    pub selected_value: String
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Report {
    pub name: String,

    #[serde(rename = "ReportHost")]
    pub report_host: Vec<ReportHost>

}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ReportHost {
    pub name: String,
    
    #[serde(rename = "HostProperties")]
    pub host_properties: HostProperties,

    #[serde(rename = "ReportItem")]
    pub report_item: Vec<ReportItem>

}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HostProperties {
    #[serde(rename = "tag")]
    pub tags: Vec<Tag>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,

    #[serde(rename = "$value")]
    pub value: String
}


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ReportItem {
    pub port: String,
    pub svc_name: String,
    pub protocol: String,
    pub severity: String,
    #[serde(rename = "pluginID")]
    pub plugin_id: String,
    #[serde(rename = "pluginName")]
    pub plugin_name: String,
    #[serde(rename = "pluginFamily")]
    pub plugin_family: String,

    pub agent: Option<String>,
    pub always_run: Option<String>,
    pub description: Option<String>,
    pub fname: Option<String>,
    pub plugin_modification_date: Option<String>,
    pub plugin_publication_date: Option<String>,
    pub risk_factor: Option<String>,
    pub script_version: Option<String>,
    pub solution: Option<String>,
    pub synopsis: Option<String>,
    pub thorough_tests: Option<String>,
    pub plugin_output: Option<String>,
    pub see_also: Option<String>,
    #[serde(serialize_with = "serialize_option_vec_as_string")]
    pub bid: Option<Vec<String>>,
    #[serde(serialize_with = "serialize_option_vec_as_string")]
    pub cve: Option<Vec<String>>,
    #[serde(serialize_with = "serialize_option_vec_as_string")]
    pub xref: Option<Vec<String>>,
    pub patch_publication_date: Option<String>,
    pub vuln_publication_date: Option<String>,
    pub exploitability_ease: Option<String>,
    pub exploit_available: Option<String>,
    pub exploit_framework_canvas: Option<String>,
    pub exploit_framework_metasploit: Option<String>,
    pub exploit_framework_core: Option<String>,
    pub metasploit_name: Option<String>,
    pub canvas_package: Option<String>,
    pub cvss_vector: Option<String>,
    pub cvss_base_score: Option<String>,
    pub cvss_temporal_score: Option<String>,
    pub change: Option<String>,
    pub plugin_type: Option<String>,
    pub plugin_version: Option<String>,
    #[serde(rename = "cm:complianceinfo")]
    pub cm_complianceinfo: Option<String>,
    #[serde(rename = "cm:complianceresult")]
    pub cm_complianceresult: Option<String>,
    #[serde(rename = "cm:complianceactualvalue")]
    pub cm_complianceactualvalue: Option<String>,
    #[serde(rename = "cm:compliancecheck-id")]
    pub cm_compliancecheck_id: Option<String>,

}

// Define a custom serialization function
fn serialize_option_vec_as_string<S>(option_vec: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match option_vec {
        Some(vec) => {
            let joined = vec.join(", ");
            serializer.serialize_str(&joined)
        },
        None => serializer.serialize_none(),
    }
}
use std::fs::File;
use std::io::prelude::*;
use quickxml_to_serde::{xml_string_to_json, Config, JsonType, JsonArray};
use pyo3::prelude::*;

use std::path::Path;
use chardet::{detect, charset2encoding};
use std::fs::OpenOptions;
use encoding::DecoderTrap;
use encoding::label::encoding_from_whatwg_label;


const ARRAY_PATHS: &'static [&'static str] = 
   &["/iati-activities/iati-activity",
    "/iati-activities/iati-activity/reporting-org/narrative",
    "/iati-activities/iati-activity/title/narrative",
    "/iati-activities/iati-activity/description",
    "/iati-activities/iati-activity/description/narrative",
    "/iati-activities/iati-activity/participating-org",
    "/iati-activities/iati-activity/participating-org/narrative",
    "/iati-activities/iati-activity/other-identifier",
    "/iati-activities/iati-activity/other-identifier/owner-org/narrative",
    "/iati-activities/iati-activity/activity-date",
    "/iati-activities/iati-activity/activity-date/narrative",
    "/iati-activities/iati-activity/contact-info",
    "/iati-activities/iati-activity/contact-info/organisation/narrative",
    "/iati-activities/iati-activity/contact-info/department/narrative",
    "/iati-activities/iati-activity/contact-info/person-name/narrative",
    "/iati-activities/iati-activity/contact-info/job-title/narrative",
    "/iati-activities/iati-activity/contact-info/telephone",
    "/iati-activities/iati-activity/contact-info/email",
    "/iati-activities/iati-activity/contact-info/website",
    "/iati-activities/iati-activity/contact-info/mailing-address",
    "/iati-activities/iati-activity/contact-info/mailing-address/narrative",
    "/iati-activities/iati-activity/recipient-country",
    "/iati-activities/iati-activity/recipient-country/narrative",
    "/iati-activities/iati-activity/recipient-region",
    "/iati-activities/iati-activity/recipient-region/narrative",
    "/iati-activities/iati-activity/location",
    "/iati-activities/iati-activity/location/location-id",
    "/iati-activities/iati-activity/location/name/narrative",
    "/iati-activities/iati-activity/location/description/narrative",
    "/iati-activities/iati-activity/location/activity-description/narrative",
    "/iati-activities/iati-activity/location/administrative",
    "/iati-activities/iati-activity/sector",
    "/iati-activities/iati-activity/sector/narrative",
    "/iati-activities/iati-activity/tag",
    "/iati-activities/iati-activity/tag/narrative",
    "/iati-activities/iati-activity/country-budget-items/budget-item",
    "/iati-activities/iati-activity/country-budget-items/budget-item/description/narrative",
    "/iati-activities/iati-activity/humanitarian-scope",
    "/iati-activities/iati-activity/humanitarian-scope/narrative",
    "/iati-activities/iati-activity/policy-marker",
    "/iati-activities/iati-activity/policy-marker/narrative",
    "/iati-activities/iati-activity/default-aid-type",
    "/iati-activities/iati-activity/budget",
    "/iati-activities/iati-activity/planned-disbursement",
    "/iati-activities/iati-activity/planned-disbursement/provider-org/narrative",
    "/iati-activities/iati-activity/planned-disbursement/receiver-org/narrative",
    "/iati-activities/iati-activity/transaction",
    "/iati-activities/iati-activity/transaction/description/narrative",
    "/iati-activities/iati-activity/transaction/provider-org/narrative",
    "/iati-activities/iati-activity/transaction/receiver-org/narrative",
    "/iati-activities/iati-activity/transaction/sector",
    "/iati-activities/iati-activity/transaction/sector/narrative",
    "/iati-activities/iati-activity/transaction/recipient-country/narrative",
    "/iati-activities/iati-activity/transaction/recipient-region/narrative",
    "/iati-activities/iati-activity/transaction/aid-type",
    "/iati-activities/iati-activity/document-link",
    "/iati-activities/iati-activity/document-link/title/narrative",
    "/iati-activities/iati-activity/document-link/description/narrative",
    "/iati-activities/iati-activity/document-link/category",
    "/iati-activities/iati-activity/document-link/language",
    "/iati-activities/iati-activity/related-activity",
    "/iati-activities/iati-activity/legacy-data",
    "/iati-activities/iati-activity/conditions/condition",
    "/iati-activities/iati-activity/conditions/condition/narrative",
    "/iati-activities/iati-activity/result",
    "/iati-activities/iati-activity/result/title/narrative",
    "/iati-activities/iati-activity/result/description/narrative",
    "/iati-activities/iati-activity/result/document-link",
    "/iati-activities/iati-activity/result/document-link/title/narrative",
    "/iati-activities/iati-activity/result/document-link/description/narrative",
    "/iati-activities/iati-activity/result/document-link/category",
    "/iati-activities/iati-activity/result/document-link/language",
    "/iati-activities/iati-activity/result/reference",
    "/iati-activities/iati-activity/result/indicator",
    "/iati-activities/iati-activity/result/indicator/title/narrative",
    "/iati-activities/iati-activity/result/indicator/description/narrative",
    "/iati-activities/iati-activity/result/indicator/document-link",
    "/iati-activities/iati-activity/result/indicator/document-link/title/narrative",
    "/iati-activities/iati-activity/result/indicator/document-link/description/narrative",
    "/iati-activities/iati-activity/result/indicator/document-link/category",
    "/iati-activities/iati-activity/result/indicator/document-link/language",
    "/iati-activities/iati-activity/result/indicator/reference",
    "/iati-activities/iati-activity/result/indicator/baseline",
    "/iati-activities/iati-activity/result/indicator/baseline/location",
    "/iati-activities/iati-activity/result/indicator/baseline/dimension",
    "/iati-activities/iati-activity/result/indicator/baseline/document-link",
    "/iati-activities/iati-activity/result/indicator/baseline/document-link/title/narrative",
    "/iati-activities/iati-activity/result/indicator/baseline/document-link/description/narrative",
    "/iati-activities/iati-activity/result/indicator/baseline/document-link/category",
    "/iati-activities/iati-activity/result/indicator/baseline/document-link/language",
    "/iati-activities/iati-activity/result/indicator/baseline/comment/narrative",
    "/iati-activities/iati-activity/result/indicator/period",
    "/iati-activities/iati-activity/result/indicator/period/target",
    "/iati-activities/iati-activity/result/indicator/period/target/location",
    "/iati-activities/iati-activity/result/indicator/period/target/dimension",
    "/iati-activities/iati-activity/result/indicator/period/target/comment/narrative",
    "/iati-activities/iati-activity/result/indicator/period/target/document-link",
    "/iati-activities/iati-activity/result/indicator/period/target/document-link/title/narrative",
    "/iati-activities/iati-activity/result/indicator/period/target/document-link/description/narrative",
    "/iati-activities/iati-activity/result/indicator/period/target/document-link/category",
    "/iati-activities/iati-activity/result/indicator/period/target/document-link/language",
    "/iati-activities/iati-activity/result/indicator/period/actual",
    "/iati-activities/iati-activity/result/indicator/period/actual/location",
    "/iati-activities/iati-activity/result/indicator/period/actual/dimension",
    "/iati-activities/iati-activity/result/indicator/period/actual/comment/narrative",
    "/iati-activities/iati-activity/result/indicator/period/actual/document-link",
    "/iati-activities/iati-activity/result/indicator/period/actual/document-link/title/narrative",
    "/iati-activities/iati-activity/result/indicator/period/actual/document-link/description/narrative",
    "/iati-activities/iati-activity/result/indicator/period/actual/document-link/category",
    "/iati-activities/iati-activity/result/indicator/period/actual/document-link/language",
    "/iati-activities/iati-activity/crs-add/other-flags",
    "/iati-activities/iati-activity/fss/forecast"];


#[pyfunction]
pub fn convert(input: String, file: Option<String>, pretty: Option<bool>, arrays: Option<Vec<String>>) -> eyre::Result<Option<String>> {


    let xml = if Path::new(&input).exists() {

        let mut fh = OpenOptions::new().read(true).open(&input).expect(
            "Could not open file",
        );
        let mut reader: Vec<u8> = Vec::new();

        // read file
        fh.read_to_end(&mut reader).expect("Could not read file");

        // detect charset of the file
        let result = detect(&reader);
        // result.0 Encode
        // result.1 Confidence
        // result.2 Language

        // decode file into utf-8
        let coder = encoding_from_whatwg_label(charset2encoding(&result.0));
        if coder.is_some() {
            let xml_contents = coder.unwrap().decode(&reader, DecoderTrap::Ignore);
            match xml_contents {
                Ok(res) => res,
                Err(res) => return Err(eyre::eyre!(res)),
            }
        } else {
            let mut xml_file = File::open(input)?;
            let mut xml_contents = String::new();
            xml_file.read_to_string(&mut xml_contents)?;
            xml_contents
        }
    } else {
        input
    };

    let mut config = Config::new_with_defaults();

    if let Some(arrays) = arrays {
        for path in arrays {
            config.json_type_overrides.insert(path.to_string(), JsonArray::Always(JsonType::Infer));
        }
    } else {
        for path in ARRAY_PATHS {
            config.json_type_overrides.insert(path.to_string(), JsonArray::Always(JsonType::Infer));
        }
    }
    config.xml_attr_prefix = "".into();

    let json = xml_string_to_json(xml, &config)?;

    if let Some(output_file) = file {
        let writer = std::io::BufWriter::new(std::fs::File::create(output_file)?);

        if pretty.is_some() && pretty.unwrap() {
            serde_json::to_writer_pretty(writer, &json)?;
        } else {
            serde_json::to_writer(writer, &json)?;
        } 
        return Ok(None)
    }

    if pretty.is_some() && pretty.unwrap() {
        Ok(Some(serde_json::to_string_pretty(&json)?))
    } else {
        Ok(Some(serde_json::to_string(&json)?))
    }

}


#[pymodule]
fn iati2json(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    Ok(())
}

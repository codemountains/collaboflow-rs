use crate::authorization::CollaboflowAuthorization;
use crate::client::document::document_contents::DocumentContents;
use crate::client::document::document_determs::DocumentDeterms;
use crate::client::document::documents::Documents;
use crate::client::form::form_parts::FormParts;
use crate::client::form::form_settings_prints::FormSettingsPrints;
use crate::client::form::forms::Forms;
use crate::client::mystatus::mydeterms::MyDeterms;
use crate::client::mystatus::mydrafts::MyDrafts;
use crate::client::mystatus::myprocesses::MyProcesses;
use crate::client::mystatus::myrequests::MyRequests;

pub mod document;
pub mod form;
pub mod mystatus;
pub mod query_params;

pub struct CollaboflowClient {
    pub documents: Documents,
    pub document_determs: DocumentDeterms,
    pub document_contents: DocumentContents,
    pub mydeterms: MyDeterms,
    pub myrequests: MyRequests,
    pub mydrafts: MyDrafts,
    pub myprocesses: MyProcesses,
    pub forms: Forms,
    pub form_parts: FormParts,
    pub form_settings_prints: FormSettingsPrints,
}

impl CollaboflowClient {
    pub fn new(base_url: &str, authorization: CollaboflowAuthorization) -> Self {
        let authorization_header = &authorization.to_string();

        // Document
        let documents = Documents::new(base_url, authorization_header);
        let document_determs = DocumentDeterms::new(base_url, authorization_header);
        let document_contents = DocumentContents::new(base_url, authorization_header);

        // MyStatus
        let mydeterms = MyDeterms::new(base_url, authorization_header);
        let myrequests = MyRequests::new(base_url, authorization_header);
        let mydrafts = MyDrafts::new(base_url, authorization_header);
        let myprocesses = MyProcesses::new(base_url, authorization_header);

        // Form
        let forms = Forms::new(base_url, authorization_header);
        let form_parts = FormParts::new(base_url, authorization_header);
        let form_settings_prints = FormSettingsPrints::new(base_url, authorization_header);

        Self {
            documents,
            document_determs,
            document_contents,
            mydeterms,
            myrequests,
            mydrafts,
            myprocesses,
            forms,
            form_parts,
            form_settings_prints,
        }
    }
}

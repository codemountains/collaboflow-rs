use crate::authorization::CollaboflowAuthorization;
use crate::client::document::document_determs::DocumentDeterms;
use crate::client::document::documents::Documents;
use crate::client::mystatus::mydeterms::MyDeterms;
use crate::client::mystatus::mydrafts::MyDrafts;
use crate::client::mystatus::myprocesses::MyProcesses;
use crate::client::mystatus::myrequests::MyRequests;

mod document;
mod mystatus;
mod query_params;

pub struct CollaboflowClient {
    pub documents: Documents,
    pub document_determs: DocumentDeterms,
    pub mydeterms: MyDeterms,
    pub myrequests: MyRequests,
    pub mydrafts: MyDrafts,
    pub myprocesses: MyProcesses,
}

impl CollaboflowClient {
    pub fn new(base_url: &str, authorization: CollaboflowAuthorization) -> Self {
        let authorization_header = &authorization.to_string();

        // Document
        let documents = Documents::new(base_url, authorization_header);
        let document_determs = DocumentDeterms::new(base_url, authorization_header);

        // MyStatus
        let mydeterms = MyDeterms::new(base_url, authorization_header);
        let myrequests = MyRequests::new(base_url, authorization_header);
        let mydrafts = MyDrafts::new(base_url, authorization_header);
        let myprocesses = MyProcesses::new(base_url, authorization_header);

        Self {
            documents,
            document_determs,
            mydeterms,
            myrequests,
            mydrafts,
            myprocesses,
        }
    }
}

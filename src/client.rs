use crate::authorization::Authorization;
use crate::client::document::document_contents::DocumentContents;
use crate::client::document::document_determs::DocumentDeterms;
use crate::client::document::document_one::Document;
use crate::client::document::documents::Documents;
use crate::client::document::documents_search::DocumentsSearch;
use crate::client::document::documents_simulation_determs::DocumentsSimulationDeterms;
use crate::client::file::files::Files;
use crate::client::form::form_parts::FormParts;
use crate::client::form::form_settings_prints::FormSettingsPrints;
use crate::client::form::forms::Forms;
use crate::client::group::group_one::Group;
use crate::client::group::groups::Groups;
use crate::client::mystatus::mydeterms::MyDeterms;
use crate::client::mystatus::mydrafts::MyDrafts;
use crate::client::mystatus::myprocesses::MyProcesses;
use crate::client::mystatus::myrequests::MyRequests;
use crate::client::title::title_one::Title;
use crate::client::title::titles::Titles;
use crate::client::user::user_one::User;
use crate::client::user::users::Users;

pub mod document;
mod file;
pub mod form;
pub mod group;
pub mod mystatus;
pub mod title;
pub mod user;

/// This is the main entry point for the Collaboflow REST API. A `Client` is used to connect to a Collaboflow.
///
/// ## Usage
///
/// ```rust
/// # use collaboflow_rs::{Authorization, CollaboflowClient};
///
/// let authorization = Authorization::with_api_key("User id", "Api key");
/// let client = CollaboflowClient::new("Collaboflow url", authorization);
/// ```
#[derive(Debug, Clone)]
pub struct CollaboflowClient {
    pub documents: Documents,
    pub document: Document,
    pub document_determs: DocumentDeterms,
    pub document_contents: DocumentContents,
    pub documents_simulation_determs: DocumentsSimulationDeterms,
    pub documents_search: DocumentsSearch,
    pub mydeterms: MyDeterms,
    pub myrequests: MyRequests,
    pub mydrafts: MyDrafts,
    pub myprocesses: MyProcesses,
    pub files: Files,
    pub forms: Forms,
    pub form_parts: FormParts,
    pub form_settings_prints: FormSettingsPrints,
    pub users: Users,
    pub user: User,
    pub groups: Groups,
    pub group: Group,
    pub titles: Titles,
    pub title: Title,
}

impl CollaboflowClient {
    pub fn new(base_url: &str, authorization: Authorization) -> Self {
        let authorization_header = &authorization.to_string();

        // Document
        let documents = Documents::new(base_url, authorization_header);
        let document = Document::new(base_url, authorization_header);
        let document_determs = DocumentDeterms::new(base_url, authorization_header);
        let document_contents = DocumentContents::new(base_url, authorization_header);
        let documents_simulation_determs =
            DocumentsSimulationDeterms::new(base_url, authorization_header);
        let documents_search = DocumentsSearch::new(base_url, authorization_header);

        // MyStatus
        let mydeterms = MyDeterms::new(base_url, authorization_header);
        let myrequests = MyRequests::new(base_url, authorization_header);
        let mydrafts = MyDrafts::new(base_url, authorization_header);
        let myprocesses = MyProcesses::new(base_url, authorization_header);

        // File
        let files = Files::new(base_url, authorization_header);

        // Form
        let forms = Forms::new(base_url, authorization_header);
        let form_parts = FormParts::new(base_url, authorization_header);
        let form_settings_prints = FormSettingsPrints::new(base_url, authorization_header);

        // User
        let users = Users::new(base_url, authorization_header);
        let user = User::new(base_url, authorization_header);

        // Group
        let groups = Groups::new(base_url, authorization_header);
        let group = Group::new(base_url, authorization_header);

        // Title
        let titles = Titles::new(base_url, authorization_header);
        let title = Title::new(base_url, authorization_header);

        Self {
            documents,
            document,
            document_determs,
            document_contents,
            documents_simulation_determs,
            documents_search,
            mydeterms,
            myrequests,
            mydrafts,
            myprocesses,
            files,
            forms,
            form_parts,
            form_settings_prints,
            users,
            user,
            groups,
            group,
            titles,
            title,
        }
    }
}

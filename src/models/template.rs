// use diesel::{Queryable, AsChangeset, Insertable, Identifiable};
use rocket::serde::Serialize;
use erased_serde::Serialize as ErasedSerialize;
// use crate::schema::template;
use crate::{
    utils::responders::CustomResponseData,
};

// #[derive(Queryable, AsChangeset, Insertable, Identifiable, Debug, Clone, PartialEq, Default)]
// #[table_name = "processes"]
pub struct Template {
    pub id: i32,
    pub name: String,
    pub yes: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TemplateData {
    pub id: i32,
    pub name: String,
    pub yes: bool,
    pub path: String,
}

impl CustomResponseData for Template {
    fn as_response(&self, path: &String) -> Box<dyn ErasedSerialize> {
        Box::new(TemplateData {
            id: self.id,
            name: self.name.to_owned(),
            yes: self.yes,
            path: format!("{}/{}", path, self.id),
        })
    }
}

pub trait MongoDoc {
    fn database_name() -> &'static str;
    fn collection_name() -> &'static str;
    fn id(&self) -> String;
    // This exists to set an internal _id field so that Mongo
    // deserialises it correctly. There might be a better way
    // to do this but I haven't found it.
    fn set_id(&mut self) {}
}

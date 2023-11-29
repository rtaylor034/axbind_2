use crate::gtypes::Brancher;
use toml;

pub type Context = Brancher<String>;
#[derive(Debug, Clone)]
pub struct TableHandle<'t> {
    pub context: Context,
    pub table: &'t toml::Table,
}
#[derive(Debug, Clone)]
pub struct TableGetError {
    pub context: Context,
    pub key: String,
    pub error: TableGetErr,
}
impl TableGetError {
    pub fn new(context: Context, key: &str, error: TableGetErr) -> TableGetError {
        TableGetError {
            context,
            key: key.to_string(),
            error,
        }
    }
}
#[derive(Debug, Clone)]
pub enum TableGetErr {
    NoKey,
    WrongType(&'static str),
}
impl std::fmt::Display for TableGetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use TableGetErr::*;
        match self.error {
            WrongType(t) => write!(
                f,
                "toml::Value for key '{}' at '{}' is of wrong type. (expected {})",
                self.key, self.context, t
            ),
            NoKey => write!(
                f,
                "Expected key '{}' at '{}', no such key exists.",
                self.key, self.context
            ),
        }
    }
}
pub type TableResult<T> = Result<T, TableGetError>;
pub trait TableResultOptional<T> {
    fn optional(self) -> TableResult<Option<T>>;
}
impl<T> TableResultOptional<T> for TableResult<T> {
    fn optional(self) -> TableResult<Option<T>> {
        match self {
            Ok(v) => Ok(Some(v)),
            Err(e) => match e.error {
                TableGetErr::NoKey => Ok(None),
                _ => Err(e),
            },
        }
    }
}
impl<'st> TableHandle<'st> {
    pub fn new_root<'t>(table: &'t toml::Table, context: String) -> TableHandle<'t> {
        TableHandle {
            table,
            context: context.into(),
        }
    }
    pub fn get_string(&self, key: &str) -> TableResult<&'st String> {
        use TableGetErr::*;
        match self.table.get(key) {
            Some(toml::Value::String(v)) => Ok(v),
            None => Err(TableGetError::new(self.context.clone(), key, NoKey)),
            _ => Err(TableGetError::new(
                self.context.clone(),
                key,
                WrongType("STRING"),
            )),
        }
    }
    pub fn get_table<'s>(&self, key: &str) -> TableResult<TableHandle<'st>> {
        use TableGetErr::*;
        match self.table.get(key) {
            Some(toml::Value::Table(v)) => Ok(TableHandle {
                table: v,
                context: self.context.with(key.to_string()),
            }),
            None => Err(TableGetError::new(self.context.clone(), key, NoKey)),
            _ => Err(TableGetError::new(
                self.context.clone(),
                key,
                WrongType("TABLE"),
            )),
        }
    }
    pub fn get_array(&self, key: &str) -> TableResult<&'st Vec<toml::Value>> {
        use TableGetErr::*;
        match self.table.get(key) {
            Some(toml::Value::Array(v)) => Ok(v),
            None => Err(TableGetError::new(self.context.clone(), key, NoKey)),
            _ => Err(TableGetError::new(
                self.context.clone(),
                key,
                WrongType("ARRAY"),
            )),
        }
    }
    pub fn get_bool(&self, key: &str) -> TableResult<bool> {
        use TableGetErr::*;
        match self.table.get(key) {
            Some(toml::Value::Boolean(v)) => Ok(*v),
            None => Err(TableGetError::new(self.context.clone(), key, NoKey)),
            _ => Err(TableGetError::new(
                self.context.clone(),
                key,
                WrongType("BOOLEAN"),
            )),
        }
    }
    ///for collecting a table of tables into a vector of TableHandles
    ///(expects every value of this table to be a table)
    pub fn collect_tables(&self) -> TableResult<Vec<(&'st String, TableHandle<'st>)>> {
        let mut o = Vec::<(&String, TableHandle)>::new();
        for (k, v) in self.table {
            o.push((
                k,
                match v {
                    toml::Value::Table(v) => TableHandle {
                        table: v,
                        context: self.context.with(k.to_string()),
                    },
                    _ => {
                        return Err(TableGetError::new(
                            self.context.clone(),
                            k,
                            TableGetErr::WrongType("TABLE"),
                        ))
                    }
                },
            ))
        }
        Ok(o)
    }
    ///traverses this table through tables of tables, given the ordered elements of 'path'
    ///(expects all values to be tables)
    pub fn traverse<I>(&self, path: I) -> TableResult<TableHandle<'st>>
    where
        I: IntoIterator<Item = &'st str>,
    {
        let mut o = self.clone();
        for key in path {
            o = o.get_table(key)?;
        }
        Ok(o)
    }
}

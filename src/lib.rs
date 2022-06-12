mod cell_renderers;
mod data_grid;
mod row_renderers;
mod value;

pub use cell_renderers::*;
pub use data_grid::*;
pub use row_renderers::*;
pub use value::Value;

pub use dioxus_table_macro::TableData;

pub trait DataGridName {
    fn data_grid_name(&self) -> &str;
}

pub struct ColumnDef {
    pub value: Value,
    pub column_title: String,
    pub header_cell_class: String,
    pub header_cell_reader: String,
    pub cell_class: String,
    pub cell_render: String,
    pub sequence_number: u8,
    pub sortable: bool,
    pub filterable: bool,
    pub skip: bool,
}

pub trait ColumnTrait {
    type DataGrid: DataGrid;

    fn def(&self) -> ColumnDef;
}

pub trait PrimaryKeyTrait {
    fn automatic(&self) -> bool;
}

pub trait PrimaryKeyIntoColumn {
    type Column: ColumnTrait;

    fn into_column(&self) -> Self::Column;

    fn from_column(col: Self::Column) -> Option<Self>
    where
        Self: Sized;
}

pub trait DataGrid: DataGridName {
    type Column: ColumnTrait;

    type PrimaryKey: PrimaryKeyTrait + PrimaryKeyIntoColumn;
}

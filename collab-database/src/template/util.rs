use crate::database::{timestamp, Database, DatabaseContext};
use crate::entity::{CreateDatabaseParams, CreateViewParams};
use crate::error::DatabaseError;
use crate::fields::Field;
use crate::rows::{CreateRowParams, RowId};
use crate::template::entity::DatabaseTemplate;
use crate::workspace_database::NoPersistenceDatabaseCollabService;
use std::sync::Arc;

pub async fn database_from_template(
  database_id: String,
  view_id: String,
  template: DatabaseTemplate,
) -> Result<Database, DatabaseError> {
  let params = create_database_params_from_template(database_id, view_id, template);
  let context = DatabaseContext {
    collab_service: Arc::new(NoPersistenceDatabaseCollabService),
    notifier: Default::default(),
  };
  let database = Database::create_with_view(params, context).await?;
  Ok(database)
}

pub fn construct_create_database_params<T>(
  database_id: String,
  view_id: String,
  template: T,
) -> Result<CreateDatabaseParams, DatabaseError>
where
  T: TryInto<DatabaseTemplate>,
  <T as TryInto<DatabaseTemplate>>::Error: ToString,
{
  let template = template
    .try_into()
    .map_err(|err| DatabaseError::ImportData(err.to_string()))?;
  let params = create_database_params_from_template(database_id, view_id, template);
  Ok(params)
}

pub(crate) fn create_database_params_from_template(
  database_id: String,
  view_id: String,
  template: DatabaseTemplate,
) -> CreateDatabaseParams {
  let inline_view_id = view_id;
  let timestamp = timestamp();

  let mut fields = vec![];
  for template_field in template.fields {
    let mut field = Field::new(
      template_field.field_id,
      template_field.name,
      template_field.field_type as i64,
      template_field.is_primary,
    );
    for (field_type, type_options) in template_field.type_options {
      field = field.with_type_option_data(field_type.type_id(), type_options);
    }
    fields.push(field);
  }

  let mut rows = vec![];
  for row_template in template.rows {
    rows.push(CreateRowParams {
      id: RowId::from(row_template.row_id),
      database_id: database_id.clone(),
      cells: row_template.cells,
      height: row_template.height,
      visibility: row_template.visibility,
      row_position: Default::default(),
      created_at: timestamp,
      modified_at: timestamp,
    });
  }

  let mut views = vec![];
  for view_template in template.views {
    views.push(CreateViewParams {
      database_id: database_id.clone(),
      view_id: inline_view_id.clone(),
      name: view_template.name,
      layout: view_template.layout,
      layout_settings: view_template.layout_settings,
      filters: view_template.filters,
      group_settings: view_template.group_settings,
      sorts: view_template.sorts,
      field_settings: Default::default(),
      created_at: timestamp,
      modified_at: timestamp,
      deps_fields: vec![],
      deps_field_setting: vec![],
    });
  }

  CreateDatabaseParams {
    database_id,
    inline_view_id,
    fields,
    rows,
    views,
  }
}

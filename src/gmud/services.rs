use chrono::Utc;
use futures_util::TryStreamExt;
use mongodb::{
    Collection, Database,
    bson::{Bson, Document, doc, oid::ObjectId},
    options::FindOptions,
};
use validator::Validate;

use crate::errors::api_error::ApiError;

use super::{
    dtos::{
        CreateGmudDto, GmudListResponseDto, GmudResponseDto, ListGmudsQueryDto, SortOrder,
        UpdateGmudDto,
    },
    models::{GmudModel, GmudStatus},
};

fn collection(db: &Database) -> Collection<GmudModel> {
    db.collection("gmuds")
}

pub async fn create_gmud(db: &Database, input: CreateGmudDto) -> Result<GmudResponseDto, ApiError> {
    input
        .validate()
        .map_err(|err| ApiError::Validation(err.to_string()))?;

    let now = Utc::now().to_rfc3339();
    let mut gmud = GmudModel {
        id: None,
        title: input.title,
        project_id: input.project_id,
        spring: input.spring,
        gmud_type: input.gmud_type,
        gmud_number: input.gmud_number,
        developer: input.developer,
        approver: input.approver,
        status: GmudStatus::Draft,
        created_at: now.clone(),
        updated_at: now,
    };

    let insert = collection(db).insert_one(&gmud).await?;
    gmud.id = insert.inserted_id.as_object_id();
    Ok(gmud.into())
}

pub async fn list_gmuds(
    db: &Database,
    query: ListGmudsQueryDto,
) -> Result<GmudListResponseDto, ApiError> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(10).clamp(1, 100);
    let skip = (page - 1) * limit;
    let filter = build_list_filter(&query)?;
    let sort = build_sort_doc(&query);

    let find_options = FindOptions::builder()
        .sort(sort)
        .skip(skip)
        .limit(limit as i64)
        .build();

    let total_items = collection(db).count_documents(filter.clone()).await?;
    let cursor = collection(db)
        .find(filter)
        .with_options(find_options)
        .await?;
    let rows: Vec<GmudModel> = cursor.try_collect().await?;
    let total_pages = if total_items == 0 {
        0
    } else {
        total_items.div_ceil(limit)
    };

    Ok(GmudListResponseDto {
        items: rows.into_iter().map(Into::into).collect(),
        page,
        limit,
        total_items,
        total_pages,
    })
}

fn build_list_filter(query: &ListGmudsQueryDto) -> Result<Document, ApiError> {
    let mut filter = doc! {};

    if let Some(status) = &query.status {
        let status_bson = mongodb::bson::to_bson(status)
            .map_err(|err| ApiError::Validation(format!("status invalido: {err}")))?;
        filter.insert("status", status_bson);
    }

    if let Some(project_id) = &query.project_id {
        if project_id.trim().is_empty() {
            return Err(ApiError::Validation(
                "projectId nao pode ser vazio".to_string(),
            ));
        }
        filter.insert("project_id", Bson::String(project_id.trim().to_string()));
    }

    Ok(filter)
}

fn build_sort_doc(query: &ListGmudsQueryDto) -> Document {
    let field = match query.sort_by.as_deref() {
        Some("title") => "title",
        Some("projectId") | Some("project_id") => "project_id",
        Some("status") => "status",
        Some("createdAt") | Some("created_at") => "created_at",
        Some("updatedAt") | Some("updated_at") => "updated_at",
        _ => "created_at",
    };

    let order = match query.sort_order {
        Some(SortOrder::Asc) => 1,
        _ => -1,
    };

    doc! { field: order }
}

pub async fn get_gmud_by_id(db: &Database, id: &str) -> Result<GmudResponseDto, ApiError> {
    let object_id = parse_object_id(id)?;
    let item = collection(db)
        .find_one(doc! {"_id": object_id})
        .await?
        .ok_or_else(|| ApiError::NotFound("gmud nao encontrada".to_string()))?;

    Ok(item.into())
}

pub async fn update_gmud(
    db: &Database,
    id: &str,
    input: UpdateGmudDto,
) -> Result<GmudResponseDto, ApiError> {
    input
        .validate()
        .map_err(|err| ApiError::Validation(err.to_string()))?;

    let object_id = parse_object_id(id)?;
    let update_doc = build_update_doc(input)?;

    let updated = collection(db)
        .find_one_and_update(doc! {"_id": object_id}, doc! {"$set": update_doc})
        .return_document(mongodb::options::ReturnDocument::After)
        .await?
        .ok_or_else(|| ApiError::NotFound("gmud nao encontrada".to_string()))?;

    Ok(updated.into())
}

pub async fn delete_gmud(db: &Database, id: &str) -> Result<(), ApiError> {
    let object_id = parse_object_id(id)?;
    let result = collection(db).delete_one(doc! {"_id": object_id}).await?;
    if result.deleted_count == 0 {
        return Err(ApiError::NotFound("gmud nao encontrada".to_string()));
    }
    Ok(())
}

fn parse_object_id(id: &str) -> Result<ObjectId, ApiError> {
    ObjectId::parse_str(id).map_err(|_| ApiError::BadRequest("id invalido".to_string()))
}

fn build_update_doc(input: UpdateGmudDto) -> Result<Document, ApiError> {
    let mut update = doc! {};

    if let Some(title) = input.title {
        update.insert("title", Bson::String(title));
    }
    if let Some(project_id) = input.project_id {
        update.insert("project_id", Bson::String(project_id));
    }
    if let Some(spring) = input.spring {
        update.insert("spring", Bson::String(spring));
    }
    if let Some(gmud_type) = input.gmud_type {
        update.insert("gmud_type", Bson::String(gmud_type));
    }
    if let Some(gmud_number) = input.gmud_number {
        update.insert("gmud_number", Bson::String(gmud_number));
    }
    if let Some(developer) = input.developer {
        update.insert("developer", Bson::String(developer));
    }
    if let Some(approver) = input.approver {
        update.insert("approver", Bson::String(approver));
    }
    if let Some(status) = input.status {
        let status_bson = mongodb::bson::to_bson(&status)
            .map_err(|err| ApiError::Validation(format!("status invalido: {err}")))?;
        update.insert("status", status_bson);
    }

    if update.is_empty() {
        return Err(ApiError::Validation(
            "nenhum campo enviado para atualizacao".to_string(),
        ));
    }

    update.insert("updated_at", Bson::String(Utc::now().to_rfc3339()));
    Ok(update)
}

#[cfg(test)]
mod tests {
    use super::build_update_doc;
    use crate::gmud::dtos::UpdateGmudDto;

    #[test]
    fn should_reject_empty_update_payload() {
        let input = UpdateGmudDto {
            title: None,
            project_id: None,
            spring: None,
            gmud_type: None,
            gmud_number: None,
            developer: None,
            approver: None,
            status: None,
        };

        let result = build_update_doc(input);
        assert!(result.is_err());
    }

    #[test]
    fn should_build_update_with_title_and_project_id() {
        let input = UpdateGmudDto {
            title: Some("GMUD Teste".to_string()),
            project_id: Some("PRJ-123".to_string()),
            spring: None,
            gmud_type: None,
            gmud_number: None,
            developer: None,
            approver: None,
            status: None,
        };

        let result = build_update_doc(input);
        assert!(result.is_ok());
    }
}

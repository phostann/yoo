use axum::{
    extract::{Json, Multipart, Path, Query, State},
    http::StatusCode,
};
use s3::{creds::Credentials, Bucket, Region};
use sea_orm::DatabaseConnection;
use std::{collections::HashSet, env};
use yoo_core::{
    ConfigFilter, GroupFilter, Mutation as MutationCore, NewConfig, NewGroup, NewTemplate, NewUser,
    Pagination, Query as QueryCore, TemplateFilter, UpdateConfig, UpdateGroup, UpdateTemplate,
};

use crate::{
    jwt::{gen_auth_body, AuthBody, AuthError, AuthPayload, Claims, ClaimsType},
    resp::{PageResponse as PageResp, Response as Resp},
    vo::UserVo,
};
use bcrypt::{hash, DEFAULT_COST};
use entity::{configs, groups, templates};

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub async fn register(
    state: State<AppState>,
    Json(payload): Json<NewUser>,
) -> Result<Json<Resp<AuthBody>>, AuthError> {
    let password = hash(payload.password, DEFAULT_COST).map_err(|_| AuthError::TokenCreation)?;

    MutationCore::create_user(&state.conn, payload.email, password, payload.nickname)
        .await
        .map_err(|_| AuthError::TokenCreation)
        .map(gen_auth_body)?
        .map(Resp::new)
        .map(Json)
}

pub async fn profile(
    claims: Claims,
    state: State<AppState>,
) -> Result<Json<Resp<UserVo>>, (StatusCode, String)> {
    match claims.claims_type {
        ClaimsType::AccessToken => QueryCore::find_user_by_email(&state.conn, claims.sub.as_ref())
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            })?
            .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))
            .map(|user| user.into())
            .map(Resp::new)
            .map(Json),
        _ => Err((StatusCode::BAD_REQUEST, "Invalid token".to_string())),
    }
}

pub async fn login(
    state: State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<Resp<AuthBody>>, AuthError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let user = QueryCore::find_user_by_email(&state.conn, payload.email.as_ref())
        .await
        .map_err(|_| AuthError::WrongCredentials)?;

    match user {
        Some(user) => {
            // check password
            let is_valid = bcrypt::verify(payload.password, user.password.as_ref())
                .map_err(|_| AuthError::WrongCredentials)?;
            if is_valid {
                gen_auth_body(user)
                    .map(Resp::new)
                    .map(Json)
                    .map_err(|_| AuthError::WrongCredentials)
            } else {
                Err(AuthError::WrongCredentials)
            }
        }
        None => Err(AuthError::WrongCredentials),
    }
}

pub async fn refresh(
    claims: Claims,
    state: State<AppState>,
) -> Result<Json<Resp<AuthBody>>, AuthError> {
    match claims.claims_type {
        ClaimsType::RefreshToken => QueryCore::find_user_by_email(&state.conn, claims.sub.as_ref())
            .await
            .map_err(|_| AuthError::WrongCredentials)?
            .map_or_else(|| Err(AuthError::WrongCredentials), gen_auth_body)
            .map(Resp::new)
            .map(Json),
        _ => Err(AuthError::InvalidToken),
    }
}

pub async fn create_group(
    _: Claims,
    state: State<AppState>,
    Json(payload): Json<NewGroup>,
) -> Result<Json<Resp<groups::Model>>, (StatusCode, String)> {
    MutationCore::create_group(&state.conn, payload)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .map(Resp::new)
        .map(Json)
}

pub async fn get_group_by_id(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Resp<Option<groups::Model>>>, (StatusCode, String)> {
    QueryCore::get_group_by_id(&state.conn, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .map(Resp::new)
        .map(Json)
}

pub async fn list_groups(
    state: State<AppState>,
    Query(filter): Query<GroupFilter>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PageResp<groups::Model>>, (StatusCode, &'static str)> {
    QueryCore::list_groups(&state.conn, pagination.page, pagination.page_size, filter)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list groups"))
        .map(|res| PageResp::new(res.0, res.1, res.2))
        .map(Json)
}

pub async fn update_group_by_id(
    _: Claims,
    state: State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateGroup>,
) -> Result<Json<Resp<groups::Model>>, (StatusCode, &'static str)> {
    MutationCore::update_group_by_id(&state.conn, id, payload)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update group by id",
            )
        })
        .map(Resp::new)
        .map(Json)
}

// delete group by id
pub async fn delete_group_by_id(
    _: Claims,
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Resp<()>>, (StatusCode, String)> {
    // check if there some configs in this group
    let config_number = QueryCore::get_config_by_group_id(&state.conn, id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete group by id".to_string(),
            )
        })?;

    if config_number != 0 {
        return Err((StatusCode::BAD_REQUEST, "Group is not empty".to_string()));
    }

    let res = MutationCore::delete_group_by_id(&state.conn, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if res.rows_affected == 1 {
        Ok(Json(Resp::new(())))
    } else {
        Err((StatusCode::NOT_FOUND, "Group not found".to_string()))
    }
}

pub async fn create_config(
    _: Claims,
    state: State<AppState>,
    Json(payload): Json<NewConfig>,
) -> Result<Json<Resp<configs::Model>>, (StatusCode, String)> {
    MutationCore::create_config(&state.conn, payload)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .map(Resp::new)
        .map(Json)
}

pub async fn get_config_by_id(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Resp<Option<configs::Model>>>, (StatusCode, String)> {
    QueryCore::get_config_by_id(&state.conn, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .map(Resp::new)
        .map(Json)
}

pub async fn update_config_by_id(
    _: Claims,
    state: State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateConfig>,
) -> Result<Json<Resp<configs::Model>>, (StatusCode, &'static str)> {
    MutationCore::update_config_by_id(&state.conn, id, payload)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update config by id",
            )
        })
        .map(Resp::new)
        .map(Json)
}

pub async fn delete_config_by_id(
    _: Claims,
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Resp<()>>, (StatusCode, String)> {
    let res = MutationCore::delete_config_by_id(&state.conn, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if res.rows_affected == 1 {
        Ok(Json(Resp::new(())))
    } else {
        Err((StatusCode::NOT_FOUND, "Config not found".to_string()))
    }
}

pub async fn list_configs(
    state: State<AppState>,
    Query(pagination): Query<Pagination>,
    Query(filter): Query<ConfigFilter>,
) -> Result<Json<PageResp<configs::Model>>, (StatusCode, &'static str)> {
    QueryCore::list_configs(&state.conn, pagination.page, pagination.page_size, filter)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list configs"))
        .map(|res| PageResp::new(res.0, res.1, res.2))
        .map(Json)
}

pub async fn list_templates(
    state: State<AppState>,
    Query(pagination): Query<Pagination>,
    Query(filter): Query<TemplateFilter>,
) -> Result<Json<PageResp<templates::Model>>, (StatusCode, &'static str)> {
    QueryCore::list_templates(&state.conn, pagination.page, pagination.page_size, filter)
        .await
        .map_err(|e| {
            tracing::debug!("Failed to list templates: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to list templates",
            )
        })
        .map(|res| PageResp::new(res.0, res.1, res.2))
        .map(Json)
}

pub async fn create_template(
    _: Claims,
    state: State<AppState>,
    Json(payload): Json<NewTemplate>,
) -> Result<Json<Resp<templates::Model>>, (StatusCode, &'static str)> {
    MutationCore::create_template(&state.conn, payload)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create template: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create template",
            )
        })
        .map(Resp::new)
        .map(Json)
}

pub async fn get_template_by_id(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Resp<Option<templates::Model>>>, (StatusCode, &'static str)> {
    QueryCore::get_template_by_id(&state.conn, id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get template by id: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get template by id",
            )
        })
        .map(Resp::new)
        .map(Json)
}

pub async fn update_template_by_id(
    _: Claims,
    state: State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTemplate>,
) -> Result<Json<Resp<templates::Model>>, (StatusCode, String)> {
    MutationCore::update_template_by_id(&state.conn, id, payload)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update template: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to update template: {}", e),
            )
        })
        .map(Resp::new)
        .map(Json)
}

pub async fn delete_template_by_id(
    _: Claims,
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Resp<()>>, (StatusCode, &'static str)> {
    let res = MutationCore::delete_template_by_id(&state.conn, id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete template: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete template",
            )
        })?;

    if res.rows_affected == 1 {
        Ok(Json(Resp::new(())))
    } else {
        Err((StatusCode::NOT_FOUND, "Template not found"))
    }
}

pub async fn get_template_tags(
    state: State<AppState>,
) -> Result<Json<Resp<HashSet<String>>>, (StatusCode, &'static str)> {
    QueryCore::get_template_tags(&state.conn)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get template tags: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get template tags",
            )
        })
        .map(Resp::new)
        .map(Json)
}

pub async fn upload(mut multipart: Multipart) -> Result<Json<Resp<String>>, (StatusCode, String)> {
    let minio_server = env::var("MINIO_SERVER").expect("MINIO_SERVER must be set");
    let minio_access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY must be set");
    let minio_secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY must be set");

    let bucket = Bucket::new(
        "yoo-config",
        Region::Custom {
            region: "".to_owned(),
            endpoint: minio_server.clone(),
        },
        Credentials {
            access_key: Some(minio_access_key),
            secret_key: Some(minio_secret_key),
            security_token: None,
            session_token: None,
            expiration: None,
        },
    )
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to link minio".to_string(),
        )
    })?
    .with_path_style();

    while let Some(file) = multipart.next_field().await.map_err(|e| {
        tracing::debug!("Failed to read file: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to upload file {}", e),
        )
    })? {
        // get file name
        let filename = file.file_name();

        // convert filename from &str to String
        let key = match filename {
            None => return Err((StatusCode::BAD_REQUEST, "No file".to_string())),
            Some(filename) => filename.to_string(),
        };

        let content_type = match file.content_type() {
            None => return Err((StatusCode::BAD_REQUEST, "No file".to_string())),
            Some(content_type) => content_type.to_string(),
        };

        let data = file.bytes().await.map_err(|e| {
            tracing::debug!("Failed to read file: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to upload file {}", e),
            )
        })?;

        let data = data.to_vec();

        let content = data.as_slice();

        let res = bucket
            .put_object_with_content_type(&key, content, &content_type)
            .await
            .map_err(|e| {
                tracing::debug!("Failed to read file: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to upload file {}", e),
                )
            })?;

        if res.status_code() == 200 {
            return Ok(Json(Resp::new(format!(
                "{}/yoo-config/{}",
                minio_server, key
            ))));
        }
    }

    Err((StatusCode::BAD_REQUEST, "No file".to_string()))
}

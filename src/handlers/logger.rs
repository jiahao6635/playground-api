// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::context::Context;
use crate::errors::Result;
use crate::services::logger::LoggerService;

// The Logging Service Handlers.

/// get logs for a playbook.
#[utoipa::path(
    get, path = "/v1/playbooks/{id}/logs",
    params(
        ("id" = Uuid, description = "The id of playbook"),
    ),
    responses(
        (status = 200, description = "Playbook logs found successfully"),
        (status = 404, description = "Playbook not found")
    ),
    tag = "Logger"
)]
pub async fn logs(Path(id): Path<Uuid>, State(ctx): State<Arc<Context>>) -> Result<impl IntoResponse> {
    LoggerService::logs(ctx, id).await;
    Ok(StatusCode::OK)
}

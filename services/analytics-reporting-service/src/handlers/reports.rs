use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, State},
    http::{
        header::{CONTENT_DISPOSITION, CONTENT_TYPE},
        HeaderValue, Response, StatusCode,
    },
};

use crate::{
    app_state::AppState,
    errors::internal_error,
    models::standings::CompetitionStandingResponse,
    pdf::standings_report::build_standings_pdf,
};

pub async fn download_competition_standings_pdf(
    Path(code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Response<Body>, (StatusCode, String)> {
    let url = format!(
        "{}/competitions/{}/standings",
        state.football_data_service_url, code
    );

    let response = state
        .http_client
        .get(&url)
        .send()
        .await
        .map_err(internal_error)?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error body".to_string());

        return Err((
            StatusCode::BAD_GATEWAY,
            format!("Football Data Service error: {} - {}", status, body),
        ));
    }

    let standings: CompetitionStandingResponse = response
        .json()
        .await
        .map_err(internal_error)?;

    let pdf_bytes = build_standings_pdf(&standings)
        .map_err(|e| internal_error(format!("Failed to build PDF: {}", e)))?;

    let filename = format!("{}_standings_report.pdf", standings.competition.code.to_lowercase());

    let mut response = Response::new(Body::from(pdf_bytes));
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/pdf"),
    );
    response.headers_mut().insert(
        CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
            .map_err(internal_error)?,
    );

    Ok(response)
}
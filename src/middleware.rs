use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};
use std::time::Instant;

/// ミドルウェア: アクセスログ
pub async fn access_log(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre processing
    let start_time = Instant::now();
    let method = req.method().to_string();
    let uri = req.uri().to_string();

    // invoke
    let res = next.call(req).await?;

    // post processing
    let status = res.status().as_u16();
    let exec_time = format!("{}ms", start_time.elapsed().as_millis());
    tracing::info!(status, method, uri, exec_time);
    Ok(res)
}

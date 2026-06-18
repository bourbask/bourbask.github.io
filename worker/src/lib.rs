use serde::{Deserialize, Serialize};
use worker::*;

const ALLOWED_ORIGINS: &[&str] = &[
    "https://bourbask.github.io",
    "https://www.bourbasquetkev.in",
];
const TO_EMAIL: &str = "bourbasquet.k@etik.com";
const MAX_NAME_LEN: usize = 100;
const MAX_EMAIL_LEN: usize = 254;
const MAX_MESSAGE_LEN: usize = 5000;
const RATE_LIMIT: u32 = 5;
const RATE_WINDOW_TTL: u64 = 3600;

#[derive(Deserialize)]
struct ContactPayload {
    name: String,
    email: String,
    message: String,
}

#[derive(Deserialize, Serialize)]
struct PushSubscription {
    endpoint: String,
    #[serde(rename = "expirationTime")]
    expiration_time: Option<u64>,
    keys: PushKeys,
}

#[derive(Deserialize, Serialize)]
struct PushKeys {
    p256dh: String,
    auth: String,
}

#[derive(Deserialize)]
struct UnsubscribePayload {
    endpoints: Vec<String>,
}

fn cors_headers(origin: &str) -> Headers {
    let h = Headers::new();
    let _ = h.set("Access-Control-Allow-Origin", origin);
    let _ = h.set("Access-Control-Allow-Methods", "POST, GET, OPTIONS");
    let _ = h.set("Access-Control-Allow-Headers", "Content-Type");
    h
}

fn allowed_origin(req: &Request) -> Option<String> {
    let origin = req.headers().get("Origin").ok()??;
    if ALLOWED_ORIGINS.contains(&origin.as_str()) {
        Some(origin)
    } else {
        None
    }
}

fn is_valid_email(email: &str) -> bool {
    let mut parts = email.splitn(2, '@');
    let local = parts.next().unwrap_or("");
    let domain = parts.next().unwrap_or("");
    !local.is_empty()
        && !domain.is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if req.method() == Method::Options {
        let origin = req.headers().get("Origin").ok().flatten().unwrap_or_default();
        return Ok(Response::builder()
            .with_headers(cors_headers(&origin))
            .empty());
    }

    let path = req.path().to_string();
    if path.starts_with("/sub/") {
        return handle_push_routes(req, env).await;
    }

    handle_contact(req, env).await
}

async fn handle_push_routes(req: Request, env: Env) -> Result<Response> {
    let path = req.path();
    if path == "/sub/subscribe" {
        subscribe_push(req, env).await
    } else if path == "/sub/subs" {
        list_subs(req, env).await
    } else if path == "/sub/unsubscribe" {
        unsubscribe_push(req, env).await
    } else {
        Response::error("Not Found", 404)
    }
}

async fn subscribe_push(mut req: Request, env: Env) -> Result<Response> {
    let origin = match allowed_origin(&req) {
        Some(o) => o,
        None => return Response::error("Forbidden", 403),
    };

    let sub: PushSubscription = match req.json().await {
        Ok(s) => s,
        Err(_) => return Response::error("Bad Request", 400),
    };

    if sub.endpoint.is_empty() || sub.keys.p256dh.is_empty() || sub.keys.auth.is_empty() {
        return Response::error("Bad Request", 400);
    }

    let kv = env.kv("PUSH_SUBS")?;
    let mut subs: Vec<PushSubscription> = kv
        .get("subscriptions")
        .text()
        .await?
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    if !subs.iter().any(|s| s.endpoint == sub.endpoint) {
        subs.push(sub);
        kv.put("subscriptions", serde_json::to_string(&subs)?)?
            .execute()
            .await?;
    }

    Response::builder()
        .with_headers(cors_headers(&origin))
        .ok("ok")
}

async fn list_subs(req: Request, env: Env) -> Result<Response> {
    let expected = env.secret("NOTIFY_SECRET")?.to_string();
    let provided = req
        .headers()
        .get("X-Notify-Secret")
        .ok()
        .flatten()
        .unwrap_or_default();

    if provided != expected {
        return Response::error("Forbidden", 403);
    }

    let kv = env.kv("PUSH_SUBS")?;
    let subs = kv
        .get("subscriptions")
        .text()
        .await?
        .unwrap_or_else(|| "[]".to_string());

    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;

    Response::builder().with_headers(headers).ok(&subs)
}

async fn unsubscribe_push(mut req: Request, env: Env) -> Result<Response> {
    let expected = env.secret("NOTIFY_SECRET")?.to_string();
    let provided = req
        .headers()
        .get("X-Notify-Secret")
        .ok()
        .flatten()
        .unwrap_or_default();

    if provided != expected {
        return Response::error("Forbidden", 403);
    }

    let payload: UnsubscribePayload = match req.json().await {
        Ok(p) => p,
        Err(_) => return Response::error("Bad Request", 400),
    };

    if payload.endpoints.is_empty() {
        return Response::builder().ok("ok");
    }

    let kv = env.kv("PUSH_SUBS")?;
    let mut subs: Vec<PushSubscription> = kv
        .get("subscriptions")
        .text()
        .await?
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    let before = subs.len();
    subs.retain(|s| !payload.endpoints.contains(&s.endpoint));

    if subs.len() < before {
        kv.put("subscriptions", serde_json::to_string(&subs)?)?
            .execute()
            .await?;
    }

    Response::builder().ok("ok")
}

async fn handle_contact(req: Request, env: Env) -> Result<Response> {
    let origin = match allowed_origin(&req) {
        Some(o) => o,
        None => return Response::error("Forbidden", 403),
    };

    if req.method() != Method::Post {
        return Response::error("Method Not Allowed", 405);
    }

    let ip = req
        .headers()
        .get("CF-Connecting-IP")
        .unwrap_or(None)
        .unwrap_or_else(|| "unknown".to_string());

    let kv = env.kv("RATE_LIMIT")?;
    let count: u32 = kv
        .get(&ip)
        .text()
        .await?
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    if count >= RATE_LIMIT {
        return Response::error("Too Many Requests", 429);
    }
    kv.put(&ip, (count + 1).to_string())?
        .expiration_ttl(RATE_WINDOW_TTL)
        .execute()
        .await?;

    let mut req = req;
    let payload: ContactPayload = match req.json().await {
        Ok(p) => p,
        Err(_) => return Response::error("Bad Request", 400),
    };

    let name = payload.name.trim().to_string();
    let email = payload.email.trim().to_string();
    let message = payload.message.trim().to_string();

    if name.is_empty() || email.is_empty() || message.is_empty() {
        return Response::error("Missing fields", 400);
    }

    if name.len() > MAX_NAME_LEN {
        return Response::error("Name too long", 400);
    }
    if email.len() > MAX_EMAIL_LEN {
        return Response::error("Email too long", 400);
    }
    if message.len() > MAX_MESSAGE_LEN {
        return Response::error("Message too long", 400);
    }

    if !is_valid_email(&email) {
        return Response::error("Invalid email", 400);
    }

    let resend_key = env.secret("RESEND_API_KEY")?.to_string();

    let body = serde_json::json!({
        "from": "Portfolio Contact <onboarding@resend.dev>",
        "to": [TO_EMAIL],
        "subject": format!("Portfolio — message de {}", name),
        "text": format!("De : {} <{}>\n\n{}", name, email, message),
    });

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", resend_key))?;
    headers.set("Content-Type", "application/json")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(serde_json::to_string(&body)?.into()));

    let resend_req = Request::new_with_init("https://api.resend.com/emails", &init)?;
    let resend_resp = Fetch::Request(resend_req).send().await?;

    if resend_resp.status_code() == 200 || resend_resp.status_code() == 201 {
        Response::builder()
            .with_headers(cors_headers(&origin))
            .ok("ok")
    } else {
        Response::error("Email delivery failed", 502)
    }
}

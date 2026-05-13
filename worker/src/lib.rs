use serde::{Deserialize, Serialize};
use worker::*;

const ALLOWED_ORIGIN: &str = "https://bourbask.github.io";
const TO_EMAIL: &str = "bourbasquet.k@etik.com";

#[derive(Deserialize)]
struct ContactPayload {
    name: String,
    email: String,
    message: String, // PGP-encrypted ciphertext
}

#[derive(Serialize)]
struct ResendPayload {
    from: String,
    to: Vec<String>,
    subject: String,
    text: String,
}

fn cors_headers() -> Headers {
    let h = Headers::new();
    let _ = h.set("Access-Control-Allow-Origin", ALLOWED_ORIGIN);
    let _ = h.set("Access-Control-Allow-Methods", "POST, OPTIONS");
    let _ = h.set("Access-Control-Allow-Headers", "Content-Type");
    h
}

#[event(fetch)]
async fn main(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // CORS preflight
    if req.method() == Method::Options {
        return Ok(Response::builder()
            .with_headers(cors_headers())
            .empty());
    }

    if req.method() != Method::Post {
        return Response::error("Method Not Allowed", 405);
    }

    let payload: ContactPayload = match req.json().await {
        Ok(p) => p,
        Err(_) => return Response::error("Bad Request", 400),
    };

    if payload.name.trim().is_empty()
        || payload.email.trim().is_empty()
        || payload.message.trim().is_empty()
    {
        return Response::error("Missing fields", 400);
    }

    let resend_key = env.secret("RESEND_API_KEY")?.to_string();

    let body = serde_json::json!({
        "from": "Portfolio Contact <onboarding@resend.dev>",
        "to": [TO_EMAIL],
        "subject": format!("Portfolio — message de {}", payload.name),
        "text": format!("De : {} <{}>\n\n{}", payload.name, payload.email, payload.message),
    });

    let mut headers = Headers::new();
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
            .with_headers(cors_headers())
            .ok("ok")
    } else {
        Response::error("Email delivery failed", 502)
    }
}

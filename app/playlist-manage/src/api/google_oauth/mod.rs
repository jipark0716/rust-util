use actix_web::{post, web};
use actix_web::web::Redirect;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("google")
          .service(redirect)
    );
}

#[post("redirect")]
async fn redirect(
    request: web::Json<()>,
) -> impl actix_web::Responder {
    Redirect::to("https://accounts.google.com/o/oauth2/v2/auth?
 scope=https%3A//www.googleapis.com/auth/drive.metadata.readonly%20https%3A//www.googleapis.com/auth/calendar.readonly&
 access_type=offline&
 include_granted_scopes=true&
 response_type=code&
 state=state_parameter_passthrough_value&
 redirect_uri=https%3A//developers.google.com/oauthplayground&
 client_id=client_id
")
}
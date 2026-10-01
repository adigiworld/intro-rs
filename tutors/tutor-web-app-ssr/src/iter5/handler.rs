use crate::errors::TutorError;
use actix_web::{Error, HttpResponse, Result, web};
use serde_json::json;

use crate::dbaccess::{get_user_record, post_new_user};

pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    ctx.insert("current_confirmation", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");
    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| TutorError::TeraError("Template error".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// pub async fn handle_register(
// tmpl: web::Data<tera::Tera>,
// app_state: web::Data<AppState>,
// params: web::Form<TutorRegisterForm>,
// )-> Result<HttpResponse, Error> {
// let mut ctx = tera::Context::new();
// let s;
// let username = params.username.clone();
// let user = get_user_record(&app_state.db, username.to_string()).await;
// let user_not_found: bool = user.is_err();
// //If user is not found in database, proceed to verification of passwords
// if user_not_found {
// if params.password != params.confirmation {
// ctx.insert("error", "Passwords do not match");
// // ...
// s = tmpl
// .render("register.html", &ctx)
// .map_err(|_| TutorError::TeraError(
// "Template error".to_string()))?;
// } else {
// let new_tutor = json!({
// "tutor_name": ...
// });
// let awc_client = awc::Client::default();
// let res = awc_client
// .post("http://localhost:3000/tutors/")
// .send_json(&new_tutor)
// .await
// .unwrap()
// .body()
// .await?;
// let tutor_response: TutorResponse = serde_json::from_str(
// &std::str::from_utf8(&res)?)?;
// s = format!("Congratulations. ...);
// // Hash the password
// let salt = b"somerandomsalt";
// let config = Config::default();
// let hash =
// argon2::hash_encoded(params.password.clone().as_bytes(),
// salt, &config).unwrap();
// let user = User {
// // ...
// };
// let _tutor_created = post_new_user(
// &app_state.db, user).await?;
// }
// } else {
// ctx.insert("error", "User Id already exists");
// // ...
// s = tmpl
// .render("register.html", &ctx)
// // ...; <2,14>
// };
// Ok(HttpResponse::Ok().content_type("text/html").body(s))
//     }


// pub async fn handle_register() -> Result<HttpResponse, Error> {
//     Ok(HttpResponse::Ok().body(""))
// }

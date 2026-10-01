use crate::errors::TutorError;
use crate::models::tutor::{NewTutor, Tutor, UpdateTutor};
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use sqlx::postgres::PgPool;

pub async fn get_all_tutors_db(pool: &PgPool) -> Result<Vec<Tutor>, TutorError> {
    // Prepare SQL statement
    let tutor_rows = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url,
tutor_profile FROM tutor_c6"
    )
    .fetch_all(pool)
    .await?;
    // Extract result
    let tutors: Vec<Tutor> = tutor_rows
        .iter()
        .map(|tutor_row| Tutor {
            tutor_id: tutor_row.tutor_id,
            tutor_name: tutor_row.tutor_name.clone(),
            tutor_pic_url: tutor_row.tutor_pic_url.clone(),
            tutor_profile: tutor_row.tutor_profile.clone(),
        })
        .collect();
    match tutors.len() {
        0 => Err(TutorError::NotFound("No tutors found".into())),
        _ => Ok(tutors),
    }
}

pub async fn get_tutor_details_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, TutorError> {
    // Prepare SQL statement
    let tutor_row = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url,
tutor_profile FROM tutor_c6 where tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map(|tutor_row| Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile,
    })
    .map_err(|_err| TutorError::NotFound("Tutor id not found".into()))?;
    Ok(tutor_row)
}

pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: NewTutor) -> Result<Tutor, TutorError> {
    let tutor_row = sqlx::query!(
        "insert into tutor_c6 (
tutor_name, tutor_pic_url, tutor_profile) values ($1,$2,$3)
returning tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        new_tutor.tutor_name,
        new_tutor.tutor_pic_url,
        new_tutor.tutor_profile
    )
    .fetch_one(pool)
    .await?;
    //Retrieve result
    Ok(Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile,
    })
}

// pub async fn update_tutor_details(
//     app_state: web::Data<AppState>,
//     web::Path(tutor_id): web::Path<i32>,
//     update_tutor: web::Json<UpdateTutor>,
// ) -> Result<HttpResponse, TutorError> {
//     update_tutor_details_db(&app_state.db, tutor_id, UpdateTutor::from(update_tutor))
//         .await
//         .map(|tutor| HttpResponse::Ok().json(tutor))
// }
// pub async fn delete_tutor(
//     app_state: web::Data<AppState>,
//     web::Path(tutor_id): web::Path<i32>,
// ) -> Result<HttpResponse, TutorError> {
//     delete_tutor_db(&app_state.db, tutor_id)
//         .await
//         .map(|tutor| HttpResponse::Ok().json(tutor))
// }

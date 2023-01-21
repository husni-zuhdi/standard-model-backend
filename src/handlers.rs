use super::DbPool;

use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse, Error};
use diesel::{PgConnection, QueryDsl};
use crate::diesel::RunQueryDsl;

use crate::models::{Particle, NewParticle, ParticlePayLoad};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/particles")]
async fn particles(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let particles = web::block(move || {
    let mut conn = pool.get()?;
    find_all_particles(&mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(particles))
}

#[post("/particles")]
async fn create_particle(pool: web::Data<DbPool>, payload: web::Json<ParticlePayLoad>) -> Result<HttpResponse, Error> {
  let particle = web::block(move || {
    let mut conn = pool.get()?;
    add_particle(&payload, &mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(particle))
}

#[get("/particles/{id}")]
async fn particle_by_id(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let particle = web::block(move || {
    let mut conn = pool.get()?;
    find_particle_by_id(id.into_inner(), &mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(particle))
}

#[put("/particles/{id}")]
async fn change_particles(id: web::Path<i32>, payload: web::Json<ParticlePayLoad>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let particle = web::block(move || {
    let mut conn = pool.get()?;
    update_particle(id.into_inner(), payload, &mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(particle))
}

#[delete("/particles/{id}")]
async fn destroy_particle(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let result = web::block(move || {
    let mut conn = pool.get()?;
    delete_particle(id.into_inner(), &mut conn)
  })
  .await?
  .map(|particle| HttpResponse::Ok().json(particle))
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(result)
}

fn find_all_particles(conn: &mut PgConnection) -> Result<Vec<Particle>, DbError> {
    use crate::schema::particles::dsl::*;

    let items = particles.load::<Particle>(conn)?;
    Ok(items)
}

fn add_particle(payload: &Json<ParticlePayLoad>, conn: &mut PgConnection) -> Result<Particle, DbError> {
    use crate::schema::particles::dsl::*;

    let new_particle = NewParticle {
        part_name: &payload.part_name,
        part_type: &payload.part_type,
        mass: &payload.mass,
        charge: &payload.charge,
        spin: &payload.spin,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(particles)
        .values(&new_particle)
        .get_result::<Particle>(conn)?;
    Ok(res)
}

fn find_particle_by_id(particle_id: i32, conn: &mut PgConnection) -> Result<Option<Particle>, DbError> {
    use crate::schema::particles::dsl::*;
    use crate::diesel::{ ExpressionMethods, OptionalExtension };

    let particle = particles
        .filter(part_id.eq(particle_id))
        .first::<Particle>(conn)
        .optional()?;
    Ok(particle)
}

fn update_particle(particle_id: i32, payload: Json<ParticlePayLoad>, conn: &mut PgConnection) -> Result<Particle, DbError> {
    use crate::schema::particles::dsl::*;
    use crate::diesel::ExpressionMethods;

    let particle = diesel::update(particles.find(particle_id))
        .set((
            part_name.eq(payload.part_name.clone()),
            part_type.eq(payload.part_type.clone()),
            mass.eq(payload.mass.clone()),
            charge.eq(payload.charge.clone()),
            spin.eq(payload.spin.clone()),
            updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .get_result::<Particle>(conn)?;
    Ok(particle)
}

fn delete_particle(particle_id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::particles::dsl::*;

    let count = diesel::delete(particles.find(particle_id)).execute(conn)?;
    Ok(count)
}
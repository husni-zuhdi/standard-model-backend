#[macro_use]
extern crate diesel;

use actix_web::{ middleware, web, App, HttpServer, HttpResponse };
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod handlers;
mod models;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default().allow_any_origin().send_wildcard();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("welcome!") }))
            .service(handlers::particles)
            .service(handlers::particle_by_id)
            .service(handlers::create_particle)
            .service(handlers::change_particles)
            .service(handlers::destroy_particle)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

// Unit Test
// curl http://localhost:8080/particles | jq
// curl http://localhost:8080/particles/17 | jq
// curl -d '{"part_name": "test", "part_type": "test", "mass": 123, "charge": "test", "spin": "test"}' -H "Content-type: application/json" -X POST http://localhost:8080/particles
// curl -d '{"part_name": "test-v1", "part_type": "test-v1", "mass": 1234, "charge": "test", "spin": "test"}' -H "Content-type: application/json" -X PUT http://localhost:8080/particles/18 | jq
// curl -X DELETE http://localhost:8080/particles/18 | jq

#[cfg(test)]
mod tests {
    use crate::models::Particle;

    use super::*;
    use actix_web::{test, web, App, http::{ StatusCode } };
    use diesel::pg::PgConnection;
    use diesel::r2d2::{self, ConnectionManager};
    use bytes::Bytes;
    use serde::{Serialize, Deserialize};
    use serde_json::json;

    #[derive(Serialize, Deserialize)]
    pub struct Particles {
        particle: Particle
    }

    fn establish_connection() -> DbPool {
        dotenv::dotenv().ok();
        
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }

    #[actix_web::test]
    async fn test_home_get() {
        let pool = establish_connection();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/", web::get().to(|| async { "Actix REST API" })),
        )
        .await;
        let req = test::TestRequest::get().uri("/").to_request();

        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);

        let result = test::read_body(res).await;
        assert_eq!(result, Bytes::from_static(b"Actix REST API"));
    }

    #[actix_web::test]
    async fn test_particles_get() {
        let pool = establish_connection();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(handlers::particles),
        )
        .await;

        let payload = json!([
            {"part_id":1,"part_type":"quark","part_name":"up","mass":2200000_i64,"charge":"2/3","spin":"1/2","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":2,"part_type":"quark","part_name":"down","mass":4700000_i64,"charge":"-1/3","spin":"1/2","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":3,"part_type":"quark","part_name":"top","mass":173100000000_i64,"charge":"2/3","spin":"1/2","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":4,"part_type":"quark","part_name":"bottom","mass":4180000000_i64,"charge":"-1/3","spin":"1/2","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":5,"part_type":"quark","part_name":"charm","mass":1280000000_i64,"charge":"2/3","spin":"1/2","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":6,"part_type":"quark","part_name":"strange","mass":96000000_i64,"charge":"-1/3","spin":"1/2","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":7,"part_type":"lepton","part_name":"electron","mass":511000_i64,"charge":"-1","spin":"1/2","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":8,"part_type":"lepton","part_name":"electron neutrino","mass":1_i64,"charge":"0","spin":"1/2","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":9,"part_type":"lepton","part_name":"muon","mass":105660000_i64,"charge":"-1","spin":"1/2","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":10,"part_type":"lepton","part_name":"muon neutrino","mass":170000_i64,"charge":"0","spin":"1/2","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":11,"part_type":"lepton","part_name":"tau","mass":1776800000_i64,"charge":"-1","spin":"1/2","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":12,"part_type":"lepton","part_name":"tau neutrino","mass":18200000_i64,"charge":"0","spin":"1/2","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":13,"part_type":"gaugeBoson","part_name":"gluon","mass":0_i64,"charge":"0","spin":"1","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":14,"part_type":"gaugeBoson","part_name":"photon","mass":0_i64,"charge":"0","spin":"1","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":15,"part_type":"gaugeBoson","part_name":"z boson","mass":91190000000_i64,"charge":"0","spin":"1","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":16,"part_type":"gaugeBoson","part_name":"w boson","mass":80433000000_i64,"charge":"+-1","spin":"1","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"},
            {"part_id":17,"part_type":"scalarBoson","part_name":"higs boson","mass":124970000000_i64,"charge":"0","spin":"0","created_at":"2023-01-20T16:01:13.007094","updated_at":"2023-01-20T16:01:13.007094"}
        ]);

        let res = test::TestRequest::get()
            .uri("/particles")
            .send_request(&app)
            .await;
        assert!(res.status().is_success());

        let result: serde_json::Value = test::read_body_json(res).await;
        assert_eq!(result, json!(payload));
    }
}
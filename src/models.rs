use serde::{Deserialize, Serialize};

use crate::schema::{ particles };

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Particle {
  pub part_id: i32,
  pub part_type: String,
  pub part_name: String,
  pub mass: i64,
  pub charge: String,
  pub spin: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = particles)]
pub struct NewParticle<'a> {
    pub part_type: &'a str,
    pub part_name: &'a str,
    pub mass: &'a i64,
    pub charge: &'a str,
    pub spin: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticlePayLoad {
    pub part_type: String,
    pub part_name: String,
    pub mass: i64,
    pub charge: String,
    pub spin: String,
}
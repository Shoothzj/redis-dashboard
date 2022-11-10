use actix_web::{HttpResponse, Responder, web};
use redis::{Commands, IntoConnectionInfo};
use serde::Deserialize;

use crate::constant;

#[derive(Deserialize)]
pub struct PutKeyReq {
    key: String,
    value: String,
}

pub async fn put_key(req: web::Json<PutKeyReq>) -> HttpResponse {
    match put_key_internal(req.0).await {
        Ok(_) => {
            HttpResponse::Created().body("")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn put_key_internal(req: PutKeyReq) -> Result<(), Box<dyn std::error::Error>> {
    if *constant::CLUSTER_ENABLE {
        let redis = redis::cluster::ClusterClient::new(constant::REDIS_CLUSTER_URL.split(",")
            .map(|s| s.to_string().into_connection_info().unwrap()).collect())?;
        let mut con = redis.get_connection()?;
        con.set(req.key, req.value)?;
        Ok(())
    } else {
        let redis = redis::Client::open(constant::REDIS_URL.to_string().into_connection_info().unwrap())?;
        let mut con = redis.get_connection()?;
        con.set(req.key, req.value)?;
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct Key {
    pub key: String,
}

pub async fn delete_key(info: web::Path<Key>) -> HttpResponse {
    match delete_key_internal(info.into_inner().key.to_owned()).await {
        Ok(_) => {
            HttpResponse::NoContent().body("")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn delete_key_internal(key: String) -> Result<(), Box<dyn std::error::Error>> {
    if *constant::CLUSTER_ENABLE {
        let redis = redis::cluster::ClusterClient::new(constant::REDIS_CLUSTER_URL.split(",")
            .map(|s| s.to_string().into_connection_info().unwrap()).collect())?;
        let mut con = redis.get_connection()?;
        con.del(key)?;
        Ok(())
    } else {
        let redis = redis::Client::open(constant::REDIS_URL.to_string().into_connection_info().unwrap())?;
        let mut con = redis.get_connection()?;
        con.del(key)?;
        Ok(())
    }
}

pub async fn get_key(info: web::Path<Key>) -> HttpResponse {
    match get_key_internal(info.into_inner().key.to_owned()).await {
        Ok(res) => {
            HttpResponse::Ok().body(res)
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

async fn get_key_internal(key: String) -> Result<String, Box<dyn std::error::Error>> {
    if *constant::CLUSTER_ENABLE {
        let redis = redis::cluster::ClusterClient::new(constant::REDIS_CLUSTER_URL.split(",")
            .map(|s| s.to_string().into_connection_info().unwrap()).collect())?;
        let mut con = redis.get_connection()?;
        let value: String = con.get(key)?;
        Ok(value)
    } else {
        let redis = redis::Client::open(constant::REDIS_URL.to_string().into_connection_info().unwrap())?;
        let mut con = redis.get_connection()?;
        let value: String = con.get(key)?;
        Ok(value)
    }
}

pub async fn get_key_list() -> Result<impl Responder, Box<dyn std::error::Error>> {
    match get_key_list_internal().await {
        Ok(keys) => {
            Ok(web::Json(keys))
        }
        Err(err) => {
            Err(err)
        }
    }
}

async fn get_key_list_internal() -> Result<(), Box<dyn std::error::Error>> {
    if *constant::CLUSTER_ENABLE {
        let redis = redis::cluster::ClusterClient::new(constant::REDIS_CLUSTER_URL.split(",")
            .map(|s| s.to_string().into_connection_info().unwrap()).collect())?;
        let mut con = redis.get_connection()?;
        let result = con.keys("*");
        match result {
            Ok(keys) => {
                Ok(keys)
            }
            Err(err) => {
                Err(err.to_string().into())
            }
        }
    } else {
        let redis = redis::Client::open(constant::REDIS_URL.to_string().into_connection_info().unwrap())?;
        let mut con = redis.get_connection()?;
        let result = con.keys("*");
        match result {
            Ok(keys) => {
                Ok(keys)
            }
            Err(err) => {
                Err(err.to_string().into())
            }
        }
    }
}

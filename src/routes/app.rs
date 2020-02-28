use warp::Filter;
use tera::Tera;
use super::Template;

pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().and()
}

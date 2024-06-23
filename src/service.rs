use actix_web::{get, post, put, delete, web, HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::body::BoxBody;

use serde::{Serialize, Deserialize};

use std::fmt::Display;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct Blog{
    pub id: u32,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
struct ErrNoId{
    message: String,
}

pub struct AppState{
    pub blogs: Mutex<Vec<Blog>>
}

impl Responder for Blog {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}

impl ResponseError for ErrNoId {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }
    
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::build(self.status_code())
        .content_type("application/json")
        .body(BoxBody::new(body))
    }
}

impl Display for ErrNoId {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{:?}", self)
   }
}
#[get("/api/v1/blogs")]
pub async fn get_blogs(data: web::Data<AppState>) -> impl Responder{
    let blogs = data.blogs.lock().unwrap();

    let response = serde_json::to_string(&(*blogs)).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

#[post("/api/v1/blog")]
pub async fn post_blog(req: web::Json<Blog>, data: web::Data<AppState>) -> impl Responder{
    let new_blog = Blog {
        id: req.id,
        title: String::from(&req.title),
        content: String::from(&req.content),
    };

    let mut blogs = data.blogs.lock().unwrap();

    let response = serde_json::to_string(&new_blog).unwrap();

    blogs.push(new_blog);

    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response)
}

#[get("/api/v1/blog/{id}")]
pub async fn get_blog(id: web::Path<u32>, data: web::Data<AppState>) -> Result<Blog, ErrNoId>{
    let blog_id: u32 = *id;
    let blogs = data.blogs.lock().unwrap();

    let blog: Vec<_> = blogs.iter()
                                .filter(|x| x.id == blog_id)
                                .collect();
    
    if !blog.is_empty() {
        Ok(Blog {
            id: blog[0].id,
            title: String::from(&blog[0].title),
            content: String::from(&blog[0].content)
        })
    } else {
        let response = ErrNoId{
            message: String::from("ticket not found")
        };
        Err(response)
    }
}

#[put("/api/v1/blog/{id}")]
pub async fn update_blog(id: web::Path<u32>, req: web::Json<Blog>, data: web::Data<AppState>) -> Result<HttpResponse, ErrNoId> {
    let blog_id: u32 = *id;

    let new_blog = Blog{
        id: req.id,
        title: String::from(&req.title),
        content: String::from(&req.content),
    };

    let mut blogs = data.blogs.lock().unwrap();

    let id_index = blogs.iter()
        .position(|x| x.id == blog_id);

    match id_index {
        Some(id) => {
            let response = serde_json::to_string(&new_blog).unwrap();
            blogs[id] = new_blog;
            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(response)
            )
        },
        None => {
            let response = ErrNoId {
                message: String::from("ticket not found")
            };
            Err(response)
        }
    }
}

#[delete("/api/v1/blog/{id}")]
pub async fn delete_blog(id: web::Path<u32>, data: web::Data<AppState>) -> Result<Blog, ErrNoId> {
    let blog_id: u32 = *id;
    let mut blogs = data.blogs.lock().unwrap();

    let id_index = blogs.iter()
        .position(|x| x.id == blog_id);

    match id_index {
        Some(id) => {
            let deleted_blog = blogs.remove(id);
            Ok(deleted_blog)
        },
        None => {
            let response = ErrNoId {
                message: String::from("ticket not found")
            };
            Err(response)
        }
    }
}

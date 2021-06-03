use rocket::{
    http::{ContentType, Status},
    Response,
};

#[catch(404)]
pub fn not_found() -> Response<'static> {
    Response::build()
        .status(Status::NotFound)
        .header(ContentType::JSON)
        .finalize()
}

#[catch(422)]
pub fn unprocessable_entity() -> Response<'static> {
    Response::build()
        .status(Status::UnprocessableEntity)
        .header(ContentType::JSON)
        .finalize()
}

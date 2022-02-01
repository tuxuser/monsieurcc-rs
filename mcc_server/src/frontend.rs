use rocket::fs::NamedFile;
use rocket::Route;
use std::io::{self};
use std::path::{Path, PathBuf};

async fn get_named_file(file: PathBuf) -> io::Result<NamedFile> {
    let page_directory_path = "./frontend/build";
    NamedFile::open(Path::new(&page_directory_path).join(file)).await
}

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    get_named_file(Path::new("index.html").to_path_buf()).await
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> io::Result<NamedFile> {
    get_named_file(file).await
}

pub(crate) fn routes() -> Vec<Route> {
    routes![index, files]
}

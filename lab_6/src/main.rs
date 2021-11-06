extern crate base64;
use rand::Rng;
use image::{Rgba, DynamicImage};
use imageproc::drawing::{draw_text_mut};
use rusttype::{Font, Scale};
use actix_web::{get,App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    let hello1 = draw_hello();
    let hello2 = draw_hello();
    let hello3 = draw_hello();
    let hello4 = draw_hello();
    let hello5 = draw_hello();
    HttpResponse::Ok().body(format!(
        "<center>
        <h1>Hello World!</h1>
        </center>
        <img src=\"data:image/png;base64,{}\"/>
        <a href=\"data:image/png;base64,{}\" download=\"hello\">
        Click here to download hello-world picture
        </a>
        <br>
        <img src=\"data:image/png;base64,{}\"/>
        <a href=\"data:image/png;base64,{}\" download=\"hello\">
        Click here to download hello-world picture
        </a>
        <br>
        <img src=\"data:image/png;base64,{}\"/>
        <a href=\"data:image/png;base64,{}\" download=\"hello\">
        Click here to download hello-world picture
        </a>
        <br>
        <img src=\"data:image/png;base64,{}\"/>
        <a href=\"data:image/png;base64,{}\" download=\"hello\">
        Click here to download hello-world picture
        </a>
        <br>
        <img src=\"data:image/png;base64,{}\"/>
        <a href=\"data:image/png;base64,{}\" download=\"hello\">
        Click here to download hello-world picture
        </a>",hello1,hello1,hello2,hello2,hello3,hello3,hello4,hello4,hello5,hello5))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// Vector of bytes
fn draw_hello() -> String
{
    let mut rng = rand::thread_rng();
    let mut image = DynamicImage::new_rgba8(640, 120);
    let mut color_codes: Vec<u8> = Vec::new();
    for _ in 0..3
    {
        color_codes.push(rng.gen_range(0..=255));
    }
    let font = Vec::from(include_bytes!("Arial Bold.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 96.0;
    let scale = Scale {
        x: height,
        y: height,
    };
    let text = "Hello, world!";
    let mut buf: Vec<u8> = Vec::new();
    draw_text_mut(&mut image, Rgba([color_codes[0], color_codes[1], color_codes[2], 255]), 20, 10, scale, &font, text);
    image.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
    return base64::encode(buf);
}
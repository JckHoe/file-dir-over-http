use warp::Filter;
use std::fs;

#[tokio::main]
async fn main() {
    // TODO Configurable path
    // TODO Separate UI elements
    // TODO Testing
    // TODO server config (port)
    // TODO Auto open webpage
    // TODO Packaging service as docker image?

    let files = warp::path::end()
        .map(move || {
            let paths = fs::read_dir("./files").unwrap();
            let mut html = String::from("<!DOCTYPE html><html><head><style>body { background-color: #222; color: #fff; font-family: Arial, sans-serif; }</style></head><body><ul>");

            for path in paths {
                if let Ok(entry) = path {
                    let file_name = entry.file_name().into_string().unwrap();
                    let link = format!("<li><a href=\"{}\" style=\"color: #fff;\">{}</a></li>", file_name, file_name);
                    html.push_str(&link);
                }
            }

            html.push_str("</ul></body></html>");
            warp::reply::html(html)
        });

    let dir = warp::fs::dir("./files");

    let routes = files.or(dir);

    println!("Serving files at http://127.0.0.1:3030/");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}


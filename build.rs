use std::io::Write;

const JS_FILES: [&str; 6] = [
    "utils.js",
    "html.js",
    "storage.js",
    "app.js",
    "envs.js",
    "apps.js",
];

fn main() {
    // panic!("{:?}", std::env::current_dir());
    let mut out_file = std::fs::File::create("wwwroot/js/app.js").unwrap();

    for file_name in JS_FILES {
        if file_name.ends_with(".js") {
            let content = std::fs::read_to_string(format!("JavaScript/{}", file_name)).unwrap();

            out_file
                .write_all(format!("// {}\n", file_name).as_bytes())
                .unwrap();

            for line in content.split('\n') {
                if line.trim().starts_with("//") {
                    continue;
                }

                out_file
                    .write_all(format!("{}\n", line).as_bytes())
                    .unwrap();
            }

            out_file.write_all("\n".as_bytes()).unwrap();
        }
    }
}

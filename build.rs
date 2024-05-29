use std::process::Command;

fn main() {
    // panic!("{:?}", std::env::current_dir());

    let mut command = Command::new("tsc");
    let result = command.output().expect("Failed to execute command");

    if result.status.success() {
        println!("tsc executed successfully");
    } else {
        //let out = std::str::from_utf8(&result.stdout.as_slice()).unwrap();
        //let err = std::str::from_utf8(result.stderr.as_slice()).unwrap();
        panic!("tsc failed to execute. Out: {:?}", result);
    }

    ci_utils::js::merge_js_files(
        &[
            "utils.js",
            "html.js",
            "storage.js",
            "app.js",
            "envs.js",
            "apps.js",
        ],
        "wwwroot/js/app.js",
    );

    /*
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
     */
}

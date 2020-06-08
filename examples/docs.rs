use {std::thread, vial::prelude::*};

routes! {
    GET "/" => |_| {
        Response::from_file("docs/index.html")
    };
}

fn main() {
    asset_dir!("docs/");

    thread::spawn(|| {
        let mut child = std::process::Command::new("watch")
            .args(&["make", "docs"])
            .spawn()
            .unwrap();
        child.wait().unwrap();
    });

    run!().unwrap();
}

use minreq;
use vial::{routes, run_once};

routes! {
    GET "/henlo" => |_| "hiya";
}

fn main() -> Result<(), minreq::Error> {
    let expected = "hiya";

    let (mut addr, thread) = run_once!();
    addr.push_str("henlo");

    let resp = minreq::get(addr).send()?;
    let resp = resp.as_str()?;
    // WARN: to prevent resource leaks join the thread after sending the request
    _ = thread.join();

    println!("Response: \"{}\"", resp);
    assert_eq!(resp, expected);

    Ok(())
}

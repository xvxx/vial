use vial::prelude::*;

routes! {
    GET "/" => index;
    GET "/info" => info;
}

fn index(_req: Request) -> &'static str {
    "<p><strong>What's your info?</strong></p>
    <form method='GET' action='info'>
        <p><label>Name: <input type='text' name='name'/></label></p>
        <p><label>Age:
            <select name='age'>
                <option value='0-18'>0-18</option>
                <option value='19-29'>19-29</option>
                <option value='30-49'>30-49</option>
                <option value='50-99'>50-99</option>
                <option value='100+'>100+</option>
            </select>
        </label></p>
        <p><input type='submit'/></p>
    </form>"
}

fn info(req: Request) -> String {
    format!(
        "<h3>Your Info</h3>
        <p>Name: {}</p>
        <p>Age: {}</p>",
        req.query("name").unwrap_or("Not Provided"),
        req.query("age").unwrap_or("Not Provided")
    )
}

fn main() {
    run!().unwrap();
}

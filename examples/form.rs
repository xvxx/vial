use vial::vial;

vial! {
    GET "/" => |_| {
        format!(r#"
<h3>vial form</h3>
<form method="POST" action="/">
    <p><label>Name: <input type="text" name="name"/></label></p>
    <p><label>Age:
        <select name="age">
            <option value="0-18">0-18</option>
            <option value="19-29">19-29</option>
            <option value="30-49">30-49</option>
            <option value="50-99">50-99</option>
            <option value="100+">100+</option>
        </select>
    </label></p>
    <p><input type="submit"/></p>
</form>
    "#).into()
    };

    POST "/" => |req| {
        format!(r#"
<h3>results</h3>
<p><b>Name:</b> {}</p>
<p><b>Age:</b> {}</p>
"#,
    req.form("name").unwrap_or(""),
    req.form("age").unwrap_or(""),
).into()
    };
}

fn main() {
    vial::run!("0.0.0.0:9999").unwrap();
}

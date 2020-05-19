use vial::vial;

vial! {
    GET "/test" => |_| "test route".into();
    GET "/" => |_| "it worked".into();
}

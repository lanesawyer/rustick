#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("server/static"))
        .launch();
}

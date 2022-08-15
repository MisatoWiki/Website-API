use rocket::{fairing::AdHoc, *};

use misato_database::{database::*, models::apiuser_model::ApiUser};
use misato_utils::settings::Settings;

mod errors;
mod fairings;
mod routes;

use routes::{admin, api, root, user};

fn init() -> AdHoc {
    AdHoc::on_ignite("Connecting to MongoDB", |rocket| async {
        let settings = Settings::init();
        match Database::init(&settings).await {
            Ok(database) => {
                // Create admin user
                let user = ApiUser::create_default(settings.admin_token.clone());
                match database.apiusermanager.create_apiuser(&user).await {
                    Ok(_) => {
                        println!("Successfully created default user.")
                    }
                    Err(err) => {
                        println!("Error whilst creating default user [{:?}]", err);
                    }
                }
                rocket.manage(database).manage(settings)
            }
            Err(error) => {
                panic!("Cannot connect to MongoDB instance:: {:?}", error)
            }
        }
    })
}

#[launch]
async fn rocket() -> _ {
    let mut routes: Vec<Route> = Vec::new();

    // Api
    routes.append(&mut routes![
        api::account::signup,
        api::account::login,
        api::account::clear_tokens,
        api::account::delete,
        api::account::check_token,
    ]);

    // Everyone
    routes.append(&mut routes![root::account::login]);

    // User
    routes.append(&mut routes![
        user::account::delete,
        user::account::clear_tokens,
        user::account::check_token
    ]);

    // Admin
    routes.append(&mut routes![admin::account::signup]);

    rocket::build().attach(init()).mount("/", routes)
}

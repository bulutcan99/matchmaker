use std::sync::Arc;

use anyhow::Error;

use matchmaker::adapter::driven::storage::db::db_connection::DB;
use matchmaker::adapter::driven::storage::db::repository::user::UserRepository;
use matchmaker::adapter::driving::presentation::http::router::{AppState, make_router};
use matchmaker::adapter::driving::presentation::http::server::Server;
use matchmaker::core::application::usecase::auth::service::UserService;
use matchmaker::shared::config::environment::Environment;
use matchmaker::shared::logger::logger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    logger::init();
    Environment::from_env()
        .load()
        .expect("Environment loading failed!");
    let db = DB::new().await?;
    let user_repository = Arc::new(UserRepository::new(Arc::clone(&db.pool)));
    // let company_repository = CompanyRepository::new(Arc::clone(&db.pool));
    let user_service = Arc::new(UserService::new(Arc::clone(&user_repository)));
    let app_state = Arc::new(AppState::new(user_service));
    let route = make_router(app_state);
    Server::bind()
        .serve(route.into_make_service())
        .await
        .unwrap();

    Ok(())
}

//usecase scenario
/*
let mut hall = Hall::new();

    let admin_user = User::new_admin(
        Some(0),
        "John".to_string(),
        "Doe".to_string(),
        "john.doe@example.com".to_string(),
        "securepasswordhash".to_string(),
    );
    let admin_worker = Worker { user: admin_user, company: None, position: None };

    hall.add_worker(admin_worker);

    // Add workers
    let user1 = User::new_user(
        Some(1),
        "John1".to_string(),
        "Doe1".to_string(),
        "john.doe1@example.com".to_string(),
        "securepasswordhash1".to_string(),
    );
    let worker1 = Worker { user: user1, company: None, position: None };
    hall.add_worker(worker1);

    let user2 = User::new_user(
        Some(2),
        "John2".to_string(),
        "Doe2".to_string(),
        "john.doe2@example.com".to_string(),
        "securepasswordhash2".to_string(),
    );
    let worker2 = Worker { user: user2, company: None, position: None };
    hall.add_worker(worker2);

    let user3 = User::new_user(
        Some(3),
        "John3".to_string(),
        "Doe3".to_string(),
        "john.doe3@example.com".to_string(),
        "securepasswordhash3".to_string(),
    );
    let worker3 = Worker { user: user3, company: None, position: None };
    hall.add_worker(worker3);

    // Adding more workers with unique users
    let user4 = User::new_user(
        Some(4),
        "John4".to_string(),
        "Doe4".to_string(),
        "john.doe4@example.com".to_string(),
        "securepasswordhash4".to_string(),
    );
    let worker4 = Worker { user: user4, company: None, position: None };
    hall.add_worker(worker4);

    let user5 = User::new_user(
        Some(5),
        "John5".to_string(),
        "Doe5".to_string(),
        "john.doe5@example.com".to_string(),
        "securepasswordhash5".to_string(),
    );
    let worker5 = Worker { user: user5, company: None, position: None };
    hall.add_worker(worker5);

    // Add tables
    hall.add_table(1);
    hall.add_table(2);
    hall.add_table(3);

    loop {
        println!("started new meet");
        hall.check_and_update();
        std::thread::sleep(Duration::from_secs(10));
    }
 */

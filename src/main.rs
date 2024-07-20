use std::time::Duration;

use matchmaker::core::domain::aggregate::hall::Hall;
use matchmaker::core::domain::aggregate::worker::Worker;
use matchmaker::core::domain::entity::user::User;

fn main() {
	let mut hall = Hall::new();
	let admin_user = User::new_admin(
		"John".to_string(),
		"Doe".to_string(),
		"john.doe@example.com".to_string(),
		"securepasswordhash".to_string(),
	);

	let admin_worker = Worker { user: admin_user, company: None, position: None };
	hall.add_worker(admin_worker);

	let user1 = User::new_user(
		"John1".to_string(),
		"Doe1".to_string(),
		"john.doe1@example.com".to_string(),
		"securepasswordhash1".to_string(),
	);
	let worker1 = Worker { user: user1, company: None, position: None };
	hall.add_worker(worker1);

	let user2 = User::new_user(
		"John2".to_string(),
		"Doe2".to_string(),
		"john.doe2@example.com".to_string(),
		"securepasswordhash2".to_string(),
	);
	let worker2 = Worker { user: user2, company: None, position: None };
	hall.add_worker(worker2);

	hall.add_table(1);
	hall.add_table(2);

	loop {
		hall.check_and_update();
		std::thread::sleep(Duration::from_secs(10));
	}
}

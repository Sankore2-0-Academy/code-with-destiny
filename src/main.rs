fn main() {
    let mut db: Vec<User> = vec![];

    // Get User name from command line
    let user_name: String = get_input("Enter name".to_string());
    
    // Get User email from command line
    let user_email: String = get_input("Enter email".to_string());
    
    // Get User age from command line
    let user_age: String = get_input("Enter age".to_string());

    // Create user object using user values
    let user = register(user_email, user_name, user_age);

    // Adding a new user object to database
    db.push(user);

    println!("{:?}", &db[0]);
}

/**
 * Create a user object using the passed in parameters as catured from user input
 */
fn register(usr_email: String, usr_name: String, usr_age: String) -> User {
 User {
  email: usr_email,
  name: usr_name,
  age: usr_age,
 }
}

/**
 * Get user input and return a String object of the keyed in value
 */
fn get_input(message: String) -> String {
 let mut input_var: String = String::new();
 println!("{} : ", &message);
 let input = std::io::stdin().read_line(&mut input_var).unwrap();
 input_var.clone()
}

/**
 * Declaration of User object
 */
#[derive(Debug)]
pub struct User {
 email: String,
 name: String,
 age: String
}
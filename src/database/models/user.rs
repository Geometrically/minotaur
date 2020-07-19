pub struct User {
    /// The internal id of the user
    pub id: i64,
    /// The username of the user
    pub username: String,
    /// The permissions bitflag for the user on the auth service
    pub permissions: i64,
    /// A base64 representation of the hash of the password
    pub password: String,
    /// The profile picture url of the user
    /// Usually hosted on the server
    pub profile_picture_url: String,
    /// The email of the account.
    pub email: String,
    /// First name of the user (IRL, Optional)
    pub first_name: Option<String>,
    /// Last Name of the user  (IRL, Optional)
    pub last_name: Option<String>,
    /// The custom display name of the user (if doesn't want login username, optional)
    pub display_name: Option<String>,
}
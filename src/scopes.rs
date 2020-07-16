use serde::{Serialize, Deserialize};
use strum_macros::EnumString;

/// The list of all accepted scopes
/// The scopes listed here are the only ones accepted.
#[derive(Serialize, Deserialize, EnumString)]
pub enum Scopes {
    /// Gives access to every single route in the API, including administrators.
    /// This scope is restricted to admins or higher.
    /// Only manually approved apps can ask for this scope.
    /// **The verification is very important!**
    #[strum(serialize="administrator")]
    Administrator,
    /// Gives access to moderation routes, with possibility to approve mods & descriptions.
    /// It also gives access to moderation tickets history, and opening/closing.
    /// This scope is restricted to moderators or higher.
    /// Only manually approved apps can ask for this scope.
    /// **The verification is very important!**
    #[strum(serialize="moderator")]
    Moderator,
    /// Main permissions for the website, allows for everything a simple user can do.
    /// It also includes oauth app creation.
    /// This scope is available to **everyone**
    /// Only manually approved apps can ask for this scope.
    /// **The verification is very important!**
    #[strum(serialize="developer")]
    Developer,
    /// Gives access to modpack / mod creation and publishing for review.
    /// This scope is available to **everyone**
    #[strum(serialize="publish")]
    Publish,
    /// Allows for creation, deletion, modification, and lecture of versions where the user have enough permissions.
    /// This scope is available to **everyone**
    #[strum(serialize="version_admin")]
    VersionAdmin,
    /// Allows for modification of versions for every mod where the user have enough permissions
    /// This scope is available for **everyone**
    #[strum(serialize="version_write")]
    VersionWrite,
    /// Allows for viewing and listing of versions for every mod owned or where the user has permissions
    /// It includes private mods.
    /// This scope is available to **everyone**
    #[strum(serialize="version_read")]
    VersionRead,
    /// Gives access to creation, deletion, modification, and lecture of mods where the user has enough permissions
    /// This scope is available to **everyone**
    #[strum(serialize="mod_admin")]
    ModAdmin,
    /// Gives access to editing of mods where the user have enough permissions.
    /// It includes private mods.
    /// This scope is available to **everyone**
    #[strum(serialize="mod_write")]
    ModWrite,
    /// Gives access to the listing of mods where the user has enough permissions.
    /// It includes private mods.
    /// This scope is available to **everyone**
    #[strum(serialize="mod_read")]
    ModRead,
    /// Gives access to the creation, deletion, modification, and lecture of modpacks where the user has enough permissions
    /// It includes private modpacks
    /// This scope is available to **everyone**
    #[strum(serialize="modpack_admin")]
    ModpackAdmin,
    /// Gives access to the editing of modpacks where the user has enough permissions
    /// It includes private modpacks
    /// This scope is available to **everyone**
    #[strum(serialize="modpack_write")]
    ModpackWrite,
    /// Gives access to the listing and read of modpacks where the user has enough permissions
    /// It includes private modpacks
    /// This scope is available to **everyone**
    #[strum(serialize="modpack_read")]
    ModpackRead,
    /// Gives access to the creation, deletion, modification, and lecture of teams where the user have enough permissions
    /// It includes invisible members of the teams where the user is a member of.
    /// Only approved apps can use this scope to prevent spamming of invitations and creation of teams.
    #[strum(serialize="team_admin")]
    TeamAdmin,
    /// Gives access to the editing of the existing teams the user has permissions on
    /// It includes invisible members of the teams the user is a member of.
    /// This scope is available to **everyone**
    #[strum(serialize="team_write")]
    TeamWrite,
    /// Gives access to the listing of the existing teams the user has permissions on
    /// It includes invisible members of the teams where the user is a member of.
    /// This scope is available to **everyone**
    #[strum(serialize="team_read")]
    TeamRead,
    /// Allows for a readonly access of the analytics data of projects the user has permissions on.
    /// This scope is available to **everyone**
    #[strum(serialize="analytics_read")]
    AnalyticsRead,
    /// Gives access to the user metadata, like name and email address, and can include private data.
    /// This scope is available to **everyone**
    #[strum(serialize="identity")]
    Identity,
    /// Gives access to the user's email only. It can be used in a Open ID Connect environment.
    /// This scope is available to **everyone**
    #[strum(serialize="email")]
    Email
}
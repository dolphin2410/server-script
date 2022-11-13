#![warn(missing_docs)]
/*!

# Server Script
Server Script is a project fork of [monun/server-script](https://github.com/monun/server-script). It aims to automate the process of downloading and configuring minecraft servers. 
Server Script uses [paper](https://papermc.io) for the server software in default, but can be used for other server softwares like SpigotMC, Vanilla Minecraft Server, etc. 

# Why Docs?
This is a documentation for people who wants to improve or fork my repository.
*/

/// The `config` crate is for managing configs
pub mod config;

/// The `web` crate is for downloading or fetching data from the web
pub mod web;

/// The `util` crate is for a convenient development, and other cool stuff
pub mod util;

/// The `backup` crate is for compressing and backup
pub mod backup;

/// The `cli` crate is for cli options for the program 
pub mod cli;

/// The `protocol` crate is for handling server url protocols
pub mod protocol;
// Lprs - A local CLI password manager
// Copyright (C) 2024  Awiteb
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/gpl-3.0.html>.

use std::num::NonZeroU64;

use clap::Args;
// use comfy_table::Table;
use regex::Regex;

use crate::{password::Passwords, LprsError, LprsResult, RunCommand};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct List {
    /// Show the clean password
    #[arg(short = 'p', long)]
    unhide_password: bool,
    /// Show the service of the password and search in it if you search
    #[arg(short = 's', long)]
    with_service: bool,
    /// Show the note of the password and search in it if you search
    #[arg(short = 'n', long)]
    with_note: bool,

    /// Return the password with spesifc index
    #[arg(short, long, value_name = "INDEX")]
    get: Option<NonZeroU64>,
    /// Search and display only matching passwords.
    ///
    /// The name and username will be searched. And service and note if included
    #[arg(short = 'e', long, value_name = "TEXT")]
    search: Option<String>,
    /// Enable regex in the search
    #[arg(short, long)]
    regex: bool,
}

impl RunCommand for List {
    fn run(&self, password_manager: Passwords) -> LprsResult<()> {
        if password_manager.passwords.is_empty() {
            Err(LprsError::Other(
                "Looks like there is no passwords to list".to_owned(),
            ))
        } else {
            if self.get.is_some() && self.search.is_some() {
                return Err(LprsError::ArgsConflict(
                    "You cannot use `--get` arg with `--search` arg".to_owned(),
                ));
            }
            if self.regex && self.search.is_none() {
                return Err(LprsError::ArgsConflict(
                    "You cannot use `--regex` without `--search` arg".to_owned(),
                ));
            }

            let mut table: Vec<Vec<String>> = Vec::new();
            // let mut table = Table::new();
            let mut header = vec!["Index", "Name", "Username", "Password"];
            if self.with_service {
                header.push("Service");
            }
            if self.with_note {
                header.push("Note");
            }
            let re = Regex::new(self.search.as_deref().unwrap_or("."))?;

            table.push(header.iter().map(|s| s.to_string()).collect::<Vec<_>>());
            // table.set_header(header);
            let passwords = password_manager
                .passwords
                .iter()
                .enumerate()
                .filter(|(idx, pass)| {
                    if let Some(index) = self.get {
                        return (idx + 1) == index.get() as usize;
                    }
                    if let Some(ref pattern) = self.search {
                        if self.regex {
                            return re.is_match(&pass.name)
                                || re.is_match(&pass.username)
                                || (self.with_service
                                    && pass.service.as_ref().is_some_and(|s| re.is_match(s)))
                                || (self.with_note
                                    && pass.note.as_ref().is_some_and(|n| re.is_match(n)));
                        } else {
                            let pattern = pattern.to_lowercase();
                            return pass.name.to_lowercase().contains(&pattern)
                                || pass.username.to_lowercase().contains(&pattern)
                                || (self.with_service
                                    && pass
                                        .service
                                        .as_ref()
                                        .is_some_and(|s| s.to_lowercase().contains(&pattern)))
                                || (self.with_note
                                    && pass
                                        .note
                                        .as_ref()
                                        .is_some_and(|n| n.to_lowercase().contains(&pattern)));
                        }
                    }

                    true
                });
            for (idx, password) in passwords {
                let hide_password = "*".repeat(password.password.chars().count());
                let idx = (idx + 1).to_string();
                let mut row = vec![
                    idx.as_str(),
                    password.name.as_str(),
                    password.username.as_str(),
                    if self.unhide_password {
                        password.password.as_str()
                    } else {
                        hide_password.as_str()
                    },
                ];
                if self.with_service {
                    row.push(password.service.as_deref().unwrap_or("Not Set"))
                }
                if self.with_note {
                    row.push(password.note.as_deref().unwrap_or("Not Set"))
                }
                // table.add_row(row);
                table.push(row.iter().map(|s| s.to_string()).collect::<Vec<_>>());
            }
            println!("{table:#?}");
            Ok(())
        }
    }
}

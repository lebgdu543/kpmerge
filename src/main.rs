use anyhow::{Context, Result};
use clap::Parser;
use keepass::{
    db::{Entry, Group, Node},
    Database, DatabaseKey,
};
use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// First database (base)
    file_a: PathBuf,

    /// Second database (added on top of A)
    file_b: PathBuf,

    /// Output file
    #[arg(short, long, default_value = "merged.kdbx")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Password for A
    let pw_a = rpassword::prompt_password(format!("Password for {}: ", cli.file_a.display()))?;

    // Password for B — Enter reuses A's password
    let pw_b_input = rpassword::prompt_password(format!(
        "Password for {} (Enter to reuse): ",
        cli.file_b.display()
    ))?;
    let pw_b = if pw_b_input.is_empty() { pw_a.clone() } else { pw_b_input };

    // Load A
    let mut file_a = File::open(&cli.file_a)?;
    let mut db_a = Database::open(&mut file_a, DatabaseKey::new().with_password(&pw_a))?;

    // Load B
    let mut file_b = File::open(&cli.file_b)?;
    let db_b = Database::open(&mut file_b, DatabaseKey::new().with_password(&pw_b))?;

    // Collect A's UUIDs before we mutate it
    let mut uuids_a = HashSet::new();
    collect_uuids(&db_a.root, &mut uuids_a);

    // Walk B and collect new entries
    let mut new_entries: Vec<Entry> = Vec::new();
    collect_new_entries(&db_b.root, &uuids_a, &mut new_entries);

    // Add them to A's root group
    for entry in new_entries {
        db_a.root.add_child(entry);
    }

    // Save
    let key_out = DatabaseKey::new().with_password(&pw_a);
    db_a.save(&mut File::create(&cli.output)?, key_out)
        .with_context(|| format!("Failed to write {}", cli.output.display()))?;

    println!("Wrote {}", cli.output.display());
    Ok(())
}

fn collect_uuids(group: &Group, set: &mut HashSet<String>) {
    for node in &group.children {
        match node {
            Node::Entry(entry) => {
                set.insert(entry.uuid.to_string());
            }
            Node::Group(subgroup) => {
                collect_uuids(subgroup, set);
            }
        }
    }
}

fn collect_new_entries(group: &Group, existing: &HashSet<String>, out: &mut Vec<Entry>) {
    for node in &group.children {
        match node {
            Node::Entry(entry) => {
                if !existing.contains(&entry.uuid.to_string()) {
                    out.push(entry.clone());
                }
            }
            Node::Group(subgroup) => {
                collect_new_entries(subgroup, existing, out);
            }
        }
    }
}

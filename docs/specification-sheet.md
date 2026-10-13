# kpmerge

*Table of Contents*

| Sections | Contents      |
| -------- | ------------- |
| Sector A | Project about |
| Sector B | How it works  |

# Section A - Project about

## 1. Main purpose

This project aim to use rust and merge 2 or more KeepassXC databases into 1 .kdbx file.

- **Crate choices.** `keepass-rs`, `clap`, `ratatui`.

# Section B - How it works ?

## 1. Merging process

Copy → read → merge → write

## 2. Detailed workflow

1. No need to copy the files cause we will not delete the files that we process so gain of computing?
2. After the user executed a command like `kp merge a.kdbx b.kdbx -o merged.kdbx`
3. We will first ask for the password of `a.kdbx` then `b.kdbx` if `b.kdbx` has the same password as `a.kdbx` user can press `enter`
4. Conflict policy flag with `keep-both` as default.
	1. If two entries has same username, password... but different uuid, refer to policy flag and versioning.
	2. If two entries have the same title but different passwords, refer to policy flag and versioning.
	3. If the same entry exists in both but one was modified more recently, refer to policy flag and versioning.
	4. same group name but different group UUIDs, process as UUID-based
	5. about deleted entries, refer to policy flag and versioning.
5. After merge, we export as `merged.kdbx` or the name specified after output `-o` option. if same name exist, we call with `X_merged.kdbx`.

> and versioning :
> v0.1 rule: two entries are considered duplicates if their UUID matches. Otherwise both are kept.
> v0.2 rule:  conflict resolution by modification date the most recent kept.

> conflict policy flag: --on-conflict=ask|keep-both|newest|oldest|a|b

## 3. Security model

- How passwords are passed (CLI prompt/file).
	- as writing the password in CLI option is dangerous, we ask/prompt for the password for each files needed and store in a variable on the code for reading and writing the file back.
	- if the user has a file for unencrypting his keepass file we can ask for this too.
- Whether decrypted data ever touches disk.
	- decrypted data will be only in ram.
- What happens if the process crashes mid-merge — is there a plaintext `.kdbx` left behind?
	- if the process crash mid-merge, we lose the data there is no recovery but as we don't burn/delete the original files we can re-do the merging.
- Memory zeroing after use.
	- passwords held in memory will be zeroed after use via the `zeroize` crate.

## 4. Non-goals

What this rust application is not for you if you want :

- A new password manager
- A GUI application for keepass
- A synchronization tool across multiple devices
- A replacement to your current password manager app.
- Not handling other formats, only keepass for v0.1

## 5. Versions

- **MVP:** merge 2 files, deduplicate by UUID, output merged file.
- **v0.2:** merge N files, conflict resolution by date.
- **v0.3:** export to Bitwarden / Proton Pass.
- **v0.4:** TUI.

## 6. Data model

Rust KeePass database file parser for KDB, KDBX3 and KDBX4, with experimental support for KDBX4 writing. This come from the original [keepass-rs](https://github.com/sseemayer/keepass-rs) repo.

## 7. Error handling

What happens when:

- A file is not a valid `.kdbx`.
	- we respond with an error code that says that the given file is not a valid file type.
- The password is wrong.
	- we respond with an error code with password mismatch
- A file is KDBX 3.x and another is 4.x.
	- we ask that given files is from 2 different keepass version and still process if yes, we do the comparaison for only valid data model present in each file. and add all non check-able.
- The output path already exists.
	- We just rename the output file name. and inform the user with the new file name.

## 8. Testing

How will you know it works?

- Unit tests for merge logic.
	- automated tests like github actions
- Integration test with two fixture `.kdbx` files.
	- Yes it will be tested.
- Round-trip test: merge → open in KeePassXC → verify.
	- I'll verify after the tests.

---


Some options listed below :

kpmerge --version
kpmerge 0.1

kpmerge -o --output merged.kdbx
kpmerge -v --verbose
kpmerge -d --debug (generate a .log file)
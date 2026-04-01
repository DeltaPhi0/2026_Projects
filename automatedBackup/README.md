# Automated recovery & Cloud sync

## Overview
A homelab is only as secure as its disaster recovery plan (speaking from personal experience..). To ensure my infrastructure can survive a drive failure or data corruption, I made a backup system.

These scripts are executed daily via cron jobs and securely archives my infrastructure to an offsite cloud provider (Google Drive, in my case). 

## Separation of concerns (The dual script)
Instead of running one monolithic backup, I split the process into two distinct processes. This ensures that a failure in one environment does not prevent the backup of the other.

1. **Docker State (`dockerBackup.sh`):** Targets my persistent container volumes. It aggressively filters out ephemeral data (like active IPC sockets, Steam shader caches, and downloading temporary files) to minimize archive size and prevent tar errors.
2. **OS State (`systemServerBackup.sh`):** Captures the host machine's configuration (`/etc`, `/boot`, `/usr`, `/var`). It utilizes `--one-file-system` to ensure the backup does not accidentally traverse into mounted external drives or virtual filesystems like `/proc` and `/sys`.

## Engineering & Edge-Case Handling
* **Exit code logic:** The `tar` command often returns an exit code of `1` if a log file changes mid read. Instead of letting the script fail, I wrote custom logic to accept exit codes `0` (Perfect) and `1` (Warnings), but fail securely on a `2` (Fatal Error).
* **Secure transport:** Offsite synchronization is handled by `rclone`. The API credentials are encrypted in a local config file and explicitly referenced in the script, ensuring no tokens are hardcoded.
* **Storage:** After a successful cloud upload, the local staging archives are automatically wiped to prevent the server's local disk from filling up.

  

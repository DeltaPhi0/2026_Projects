# Automated recovery and cloud sync

## Overview
A homelab is only as secure as its disaster recovery plan (speaking from personal experience..). To ensure my infrastructure can survive a drive failure or data corruption, I made a backup system.

These scripts are executed daily via cron jobs and securely archives my system to an offsite cloud provider (Google Drive, in my case). 

## Separation of concerns (The dual script)
Instead of running one monolithic backup, I split the process into two distinct processes. This ensures that a failure in one environment does not prevent the backup of the other.

1. **Docker State (`dockerBackup.sh`):** Targets my persistent container volumes. It aggressively filters out ephemeral data (like active IPC sockets, Steam shader caches, and downloading temporary files) to minimize archive size and prevent tar errors.
2. **OS State (`systemServerBackup.sh`):** Captures the host machine's configuration (`/etc`, `/boot`, `/usr`, `/var`). It utilizes `--one-file-system` to ensure the backup does not accidentally traverse into mounted external drives or virtual filesystems like `/proc` and `/sys`.

## Engineering and edge case handling
* **Exit code logic:** The `tar` command often returns an exit code of `1` if a log file changes mid read. Instead of letting the script fail, I wrote custom logic to accept exit codes `0` (Perfect) and `1` (Warnings), but fail securely on a `2` (Fatal Error).
* **Secure transport:** Offsite synchronization is handled by `rclone`. The API credentials are encrypted in a local config file and explicitly referenced in the script, ensuring no tokens are hardcoded.
* **Storage:** After a successful cloud upload, the local staging archives are automatically wiped to prevent the server's local disk from filling up.

  
### 1. dockerBackup.sh

```bash
#!/bin/bash
# dockerBackup.sh - Archives persistent Docker files and syncs offsite.

# CONFIG
SOURCE_DIR="docker"              
PARENT_DIR="/home/user"         
DEST_LOCAL="/home/user/backup/dockerBackup"
FILENAME="dockerBackup_$(date +%Y-%m-%d).tar.gz"

# Rclone Config
REMOTE_NAME="gdrive"             
REMOTE_DEST="$REMOTE_NAME:DockerBackups"

mkdir -p "$DEST_LOCAL"

# THE BACKUP

tar -czf "$DEST_LOCAL/$FILENAME" \
  --warning=no-file-changed \
  --exclude="" \
  -C "$PARENT_DIR" "$SOURCE_DIR"
#input here whatever folders or files you do not want backed up, there can be multiple --exclude arguments

# Capture the exit code
EXIT_CODE=$?

# Logic: 0 = Success, 1 = Warnings (like 'socket ignored' or 'file changed')
if [ $EXIT_CODE -eq 0 ] || [ $EXIT_CODE -eq 1 ]; then
    sudo -u user rclone move "$DEST_LOCAL/$FILENAME" "$REMOTE_DEST/" --progress

    if [ $? -eq 0 ]; then
        echo "Upload complete. Cleaning up"
        sudo -u user rclone delete "$REMOTE_DEST/"
        sudo rm "$DEST_LOCAL/dockerBackup_*"
    else
        echo "rclone upload failed"
        exit 1
    fi
else
    echo "Tar failed with exit code $EXIT_CODE."
    exit 1
fi
```

---

### 2. systemServerBackup.sh

```bash
#!/bin/bash
# systemServerBackup.sh - Archives OS config state and syncs offsite.

# CONFIG
DEST_LOCAL="/home/user/backup/fullSystemServer"
FILENAME="fullServerBackup_$(date +%Y-%m-%d).tar.gz"

# Rclone Config
REMOTE_NAME="gdrive"             
REMOTE_DEST="$REMOTE_NAME:serverBackups"

mkdir -p "$DEST_LOCAL"

# THE BACKUP

tar -czf "$DEST_LOCAL/$FILENAME" \
  --warning=no-file-changed \
  --exclude="" \
  /home /usr /var /snap /opt /boot /etc
#input here whatever folders or files you do not want backed up, there can be multiple --exclude arguments

# Capture the exit code
EXIT_CODE=$?

# Logic: 0 = Success, 1 = Warnings (like 'socket ignored' or 'file changed')
if [ $EXIT_CODE -eq 0 ] || [ $EXIT_CODE -eq 1 ]; then
    sudo -u user rclone move "$DEST_LOCAL/$FILENAME" "$REMOTE_DEST/" --progress

    if [ $? -eq 0 ]; then
        echo "Upload complete. Cleaning up"
        sudo -u user rclone delete "$REMOTE_DEST/"
        sudo rm "$DEST_LOCAL/fullServerBackup_*"
    else
        echo "rclone upload failed"
        exit 1
    fi
else
    echo "Tar failed with exit code $EXIT_CODE."
    exit 1
fi
```


# Projects

> This repository is meant for anyone curious enough to look into my thought processes and my self-imposed goals. Sometimes I may shoot myself in the foot, but that's okay!

Welcome to my central project repository. I am an aspiring Cybersecurity Analyst, and this space serves as a living record of my technical journey, my homelab evolution, and my custom automation scripts. 

I believe the best way to learn complex systems is to build them, break them, and figure out how to put them back together securely. If you are a hiring manager, a fellow enthusiast, or just passing by, you will find my practical implementations of Linux administration, network security, and infrastructure as code here.

---

## Directory Index

Below are the active projects and systems currently documented in this repository. Click on any folder to read its specific documentation, architecture layout, and source code.

### 1. [Remote Boot & LUKS Unlocker](./remoteBoot)
A custom workflow designed to securely manage an encrypted, headless server. It features Wake-on-LAN triggering via a VPN gateway and a secure Dropbear SSH implementation within the initramfs to remotely pass LUKS decryption keys during the boot sequence.

### 2. [Automated Cloud Backup Service](./automatedBackup)
A custom Bash automation service that ensures my critical homelab data is never lost. It handles the compression of system configurations and Docker files into timestamped archives, followed by secure, automated synchronization to an off-site cloud provider (in my case, google's).

### 3. [Docker Infrastructure as Code](./docker)
A sanitized look at my containerized homelab environment. This section breaks down how I use Docker Compose to manage a fleet of over 20 active containers, including secure routing via Nginx Proxy Manager, internal network isolation, and database management.

---

## My Methodology

When building these projects, I try to adhere to the following principles:
* **Security First:** Moving away from default credentials, utilizing key based authentication, and encrypting data at rest.
* **Automation:** If I have to do it more than twice, I need to write a script for it.
* **Documentation:** Writing down my methodology so that the future me (or anyone reading this) can understand exactly why a specific decision was made.

## Connect With Me
If you have feedback on my code, suggestions for better security practices, or just want to talk about homelabs and anything else really, feel free to reach out via the contact links on my main GitHub profile.

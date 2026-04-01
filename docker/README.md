# Containerized infrastructure & microservices

## Overview
My homelab is entirely containerized. To maintain high availability and prevent single points of failure, I manage my infrastructure using a microservices approach across 20+ separate Docker compose stacks. 

This repository segment outlines my container architecture, my security philosophy regarding Docker networks, and an exact example of my deployment code.

## Network Security & Routing Architecture

Every container in my stack adheres to strict network isolation policies. Backend databases never touch the host network, and all external traffic is routed exclusively through a single reverse proxy (Nginx Proxy Manager) with forced SSL/TLS.


## Services
Currently, I manage over 20 active containers spread across multiple directories. Here is a high level breakdown of the core services operating on the server:

| Category | Services Deployed | Purpose |
| :--- | :--- | :--- |
| **Edge, Routing** | NPM, Pi-hole | DNS resolution, ad-blocking, and SSL reverse proxying. |
| **Monitoring, CI/CD** | Watchtower, Uptime Kuma, Dozzle, Dashdot, Deunhealth | Automated image patching, uptime alerting, container health checks, and live log monitoring. |
| **Data, Productivity** | Immich, Trilium, Anki Sync, Nextcloud, Vaultwarden | Machine learning photo backups, knowledge management, cloud storage, and secure credential management. |
| **Media, Education** | The "Arr" Stack, Jellyseerr, Navidrome, Kavita, Audiobookshelf, Moodle | Automated media acquisition, indexing, and self hosted e-learning. |
| **Development** | Code-Server | Browser based VS Code environment for remote scripting. |

## Infrastructure as Code (Showcase: 4gaBoards)
Below is my actual configuration for deploying a project management board. This one is one of my personal favorites, so I really do recommend using it :) 

**Key decisions:**

1. **closed space databases:** The `db` container only exists on the `boards-network`. No ports are bound to the host, making it completely invisible to the outside world.
2. **Reverse proxy:** Port bindings on the application container are completely removed. Access is only permitted through the external `proxy-net` via Nginx Proxy Manager (`TRUST_PROXY: "true"`).
3. **Lifecycle management:** The `db` container includes a specific label to explicitly block Watchtower from auto updating it, preventing unexpected schems changes or database corruption.
4. **Prevention:** The application relies on a strict healthcheck ping (`pg_isready`) before it is allowed to start, preventing crash loops during server reboots.

```yaml
services:
  db:
    image: postgres:16-alpine
    container_name: 4gaboards-db
    restart: always
    networks:
      - boards-network
    volumes:
      - db-data:/var/lib/postgresql/data
    environment:
      POSTGRES_DB: 4gaBoards
      POSTGRES_PASSWORD: ${DB_PASSWORD}
      POSTGRES_INITDB_ARGS: '-A scram-sha-256'
    labels:
      - "com.centurylinklabs.watchtower.enable=false"
    healthcheck:
      test: ['CMD-SHELL', 'pg_isready -U postgres -d 4gaBoards']
      interval: 5s
      timeout: 5s
      retries: 5

  4gaBoards:
    image: ghcr.io/rargames/4gaboards:latest
    container_name: 4gaboards-app
    restart: always
    networks:
      - boards-network  # Private talk to DB
      - proxy-net       # Public talk to NPM
    volumes:
      - user-avatars:/home/user/Pictures/avatars
      - project-background-images:/home/user/Pictures/projBackground
      - attachments:/home/user/Pictures/attachments
    environment:
      BASE_URL: [https://boards.mydomain.com](https://boards.mydomain.com)
      SECRET_KEY: ${SECRET_KEY}
      DATABASE_URL: postgresql://postgres:${DB_PASSWORD}@4gaboards-db/4gaBoards
      NODE_ENV: production
      TRUST_PROXY: "true"
    depends_on:
      db:
        condition: service_healthy

volumes:
  user-avatars:
  project-background-images:
  attachments:
  db-data:

networks:
  boards-network:
  proxy-net:
    external: true

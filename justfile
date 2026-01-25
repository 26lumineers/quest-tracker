up:
    podman compose -f docker-compose-dev.yml up -d
down:
    podman compose -f docker-compose-dev.yml down
db_exec:
    podman exec -it quest-tracker-db psql -U quest_admin -d quest_tracker

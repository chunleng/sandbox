# Diesel CLI

Tool for using `diesel-cli` without installing the binary locally.

## Status

Working

## Getting Started

```bash
MIGRATION_PATH="/path/to/folder" DATABASE_URL="postgres://postgres:password@localhost:5432/postgres" docker-compose run --rm diesel-cli migration run
MIGRATION_PATH="/Users/chunlenglim/Documents/ggg/taskmunch/app/frontend/db" DATABASE_URL="sqlite:///Users/chunlenglim/Documents/ggg/taskmunch/app/frontend/db/test.db" docker-compose run --rm diesel-cli migration run
MIGRATION_PATH="/Users/chunlenglim/Documents/ggg/taskmunch/app/frontend/db" docker-compose run --rm diesel-cli migration generate "create_tasks"
```

Note that for migration path, it should point to the folder that contains the
migration folder and not the folder itself.

This is because the auto-generated schema is likely to end up outside the
migration folder, and we don't want what is changed in the schema to not be
reflected in the host machine.

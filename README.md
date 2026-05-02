# mbopml
Export Micro.Blog following as OPML

## Python setup (uv)

Install dependencies:

```bash
uv sync
```

Run the script:

```bash
uv run python mbopml.py --api-key <API_KEY> --username <USERNAME> --format xml
```

Update dependencies to the latest available versions:

```bash
uv lock --upgrade
uv sync
```

## Migration from Pipenv to uv

This project now uses uv for dependency management and locking.

Common command mappings:

- `pipenv install` -> `uv sync`
- `pipenv run python mbopml.py ...` -> `uv run python mbopml.py ...`
- `pipenv update` -> `uv lock --upgrade && uv sync`

If you still have old Pipenv files or environment state locally, remove them:

```bash
rm -f Pipfile Pipfile.lock
rm -rf .venv
uv sync
```

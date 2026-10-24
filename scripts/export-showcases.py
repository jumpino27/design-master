#!/usr/bin/env python3
"""Export the four approved projects without settings, credentials, or request logs."""

import json
from pathlib import Path
import re
import shutil
import sqlite3
import sys


ROOT = Path(__file__).resolve().parents[1]
PROJECTS = [
    ("Nexora", "nexora"),
    ("Elevate Studio", "elevate-studio"),
    ("MetricFlow", "metricflow"),
    ("VYBE", "vybe"),
]


def export(data_dir):
    connection = sqlite3.connect(f"file:{data_dir / 'design-gen.sqlite'}?mode=ro", uri=True)
    connection.row_factory = sqlite3.Row
    catalog = []
    for name, slug in PROJECTS:
        projects = connection.execute("SELECT * FROM projects WHERE name = ?", (name,)).fetchall()
        if len(projects) != 1:
            raise ValueError(f"Expected one project named {name}")
        project = dict(projects[0])
        project_id = project["id"]
        if connection.execute("SELECT count(*) FROM assets WHERE project_id = ?", (project_id,)).fetchone()[0]:
            raise ValueError(f"Review uploaded reference assets before exporting {name}")

        tables = {"projects": [project]}
        for table in ["brand_versions", "threads", "runs", "outputs"]:
            tables[table] = [dict(row) for row in connection.execute(
                f"SELECT * FROM {table} WHERE project_id = ? ORDER BY created_at, id", (project_id,)
            )]
        if any(row["status"] != "completed" for row in tables["runs"] + tables["outputs"]):
            raise ValueError(f"Finish all runs and outputs before exporting {name}")

        replacements = {project_id: f"example-{slug}"}
        prefixes = {"brand_versions": "brand", "threads": "thread", "runs": "run", "outputs": "out"}
        for table, prefix in prefixes.items():
            for index, row in enumerate(tables[table], 1):
                replacements[row["id"]] = f"example-{prefix}-{slug}" + (f"-{index}" if table in ["outputs", "brand_versions"] else "")

        def portable(value):
            if not isinstance(value, str):
                return value
            for original, replacement in replacements.items():
                value = value.replace(original, replacement)
            value = value.replace(str(data_dir), "[local workspace]").replace(str(ROOT), "[repository]")
            if re.search(r"sk-or-v1-|ghp_|gho_|/Users/|/Volumes/", value):
                raise ValueError(f"Review private data in {name} before publishing")
            return value

        folder = ROOT / "assets" / "showcases" / slug
        folder.mkdir(parents=True, exist_ok=True)
        images = []
        for index, row in enumerate(tables["outputs"], 1):
            source = data_dir / row["original_rel_path"]
            image_name = f"{index:02d}.png"
            shutil.copyfile(source, folder / image_name)
            row["original_rel_path"] = f"showcases/{slug}/{image_name}"
            row["thumbnail_rel_path"] = f"showcases/{slug}/{index:02d}.webp"
            images.append((replacements[row["id"]], image_name))
            for key in ["provider_request_id", "error_message", "save_journal_path"]:
                row[key] = None
            row["usage_json"] = "{}"
            row["actual_cost_micros"] = 0
            row["estimated_cost_micros"] = 0

        project["is_example"] = 1
        for table in ["brand_versions", "runs"]:
            for row in tables[table]:
                key = "brand_json" if table == "brand_versions" else "brand_snapshot_json"
                brand = json.loads(row[key])
                brand["target_context"]["repository_root"] = ""
                row[key] = json.dumps(brand, ensure_ascii=False)
        for row in tables["runs"]:
            row.update(idempotency_key=None, actual_cost_micros=0, estimated_cost_micros=0,
                       cost_is_unknown=0, error_message=None)

        # Keep a concise public brief rather than the creator's conversation history.
        run = tables["runs"][0]
        tables["thread_messages"] = [
            {"id": f"example-message-{slug}-1", "thread_id": replacements[run["thread_id"]],
             "role": "user", "content": project["description"], "created_at": run["created_at"]},
            {"id": f"example-message-{slug}-2", "thread_id": replacements[run["thread_id"]],
             "role": "assistant", "content": f"{name} showcase, including the finished designs and extracted assets. Open any image to inspect it or copy its implementation prompt.",
             "linked_run_id": replacements[run["id"]], "created_at": run["completed_at"]},
        ]

        statements = []
        for table, rows in tables.items():
            for row in rows:
                columns = ", ".join(row)
                values = ", ".join(connection.execute("SELECT quote(?)", (portable(value),)).fetchone()[0] for value in row.values())
                statements.append(f"INSERT INTO {table} ({columns}) VALUES ({values});")
        (folder / "project.sql").write_text("\n".join(statements) + "\n")
        image_entries = ",\n".join(
            f'            ("{output_id}", include_bytes!("../../../assets/showcases/{slug}/{image_name}"))'
            for output_id, image_name in images
        )
        catalog.append(f'''    Showcase {{
        id: "example-{slug}",
        sql: include_str!("../../../assets/showcases/{slug}/project.sql"),
        images: &[
{image_entries},
        ],
    }}''')
        print(f"Exported {name}: {len(images)} images")

    (ROOT / "crates/server/src/showcase_catalog.rs").write_text(
        "// Generated by scripts/export-showcases.py.\nconst SHOWCASES: &[Showcase] = &[\n" + ",\n".join(catalog) + ",\n];\n"
    )


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("Usage: python3 scripts/export-showcases.py <workspace-data-directory>")
    export(Path(sys.argv[1]).resolve())

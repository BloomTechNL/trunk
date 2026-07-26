#!/usr/bin/env python3
import sys
import json
import subprocess
import os
from collections import Counter

# Simple assumption: Script is run from the project root
PROJECT_DIR = os.getcwd()
BASELINE_FILE = os.path.join(PROJECT_DIR, ".clippy_baseline.json")

def scan_codebase():
    """Runs clippy and extracts lint fingerprints along with their human-readable output."""
    cmd = [
        "cargo", "clippy",
        "--all-targets",
        "--all-features",
        "--message-format=json",
        "--",
        "-W", "clippy::all",
        "-W", "clippy::pedantic",
        "-W", "clippy::nursery",
        "-W", "clippy::cargo",
        "-W", "clippy::unwrap_used",
        "-A", "clippy::missing_errors_doc"
    ]

    result = subprocess.run(cmd, capture_output=True, text=True, cwd=PROJECT_DIR)

    lint_instances = []
    has_compiler_error = False

    for line in result.stdout.splitlines():
        if not line.strip():
            continue
        try:
            data = json.loads(line)
            if data.get("reason") == "compiler-message":
                msg = data.get("message", {})
                level = msg.get("level")
                code_dict = msg.get("code")

                if level in ["warning", "error"] and code_dict is not None:
                    lint_code = code_dict.get("code")
                    message_text = msg.get("message", "")
                    rendered = msg.get("rendered", "")

                    spans = msg.get("spans", [])
                    file_name = spans[0].get("file_name", "unknown") if spans else "unknown"

                    fingerprint = f"{file_name}||{lint_code}||{message_text}"

                    lint_instances.append({
                        "fingerprint": fingerprint,
                        "rendered": rendered
                    })

                if level == "error" and code_dict is None:
                    has_compiler_error = True
        except json.JSONDecodeError:
            continue

    if result.returncode != 0 and not lint_instances:
        has_compiler_error = True

    return lint_instances, has_compiler_error, result.stderr

def load_baseline():
    if not os.path.exists(BASELINE_FILE):
        return None
    try:
        with open(BASELINE_FILE, "r") as f:
            return json.load(f)
    except Exception:
        return None

def save_baseline(lint_instances):
    fingerprints = [item["fingerprint"] for item in lint_instances]
    counts = dict(Counter(fingerprints))
    with open(BASELINE_FILE, "w") as f:
        json.dump(counts, f, indent=2, sort_keys=True)

def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--init":
        lint_instances, compile_err, stderr = scan_codebase()
        if compile_err:
            print("❌ Cannot initialize baseline: Code does not compile.", file=sys.stderr)
            print(stderr, file=sys.stderr)
            sys.exit(2)
        save_baseline(lint_instances)
        print(f"✅ Baseline initialized with {len(lint_instances)} existing lints.")
        sys.exit(0)

    baseline_counts = load_baseline()
    if baseline_counts is None:
        lint_instances, compile_err, _ = scan_codebase()
        save_baseline(lint_instances)
        print(f"⚠️ No baseline found. Initialized baseline with {len(lint_instances)} lints.")
        sys.exit(0)

    current_instances, compile_err, stderr = scan_codebase()

    if compile_err:
        print("❌ CLIPPY RATCHET FAILURE: You broke compilation! Code must compile cleanly.", file=sys.stderr)
        print(stderr, file=sys.stderr)
        sys.exit(2)

    current_fingerprints = [item["fingerprint"] for item in current_instances]
    current_counts = Counter(current_fingerprints)

    new_violations = []

    for item in current_instances:
        fp = item["fingerprint"]
        if current_counts[fp] > baseline_counts.get(fp, 0):
            if item["rendered"] not in new_violations:
                new_violations.append(item["rendered"])

    if new_violations:
        print("❌ CLIPPY RATCHET FAILURE: You introduced new lint violations!\n", file=sys.stderr)
        print("Please resolve the following specific issues:\n", file=sys.stderr)
        for rendered_error in new_violations:
            print(rendered_error, file=sys.stderr)
        sys.exit(2)

    if len(current_instances) < sum(baseline_counts.values()):
        save_baseline(current_instances)
        print(f"📉 SUCCESS: Codebase is cleaner! Baseline ratcheted down to {len(current_instances)} lints.")
    else:
        print("✅ SUCCESS: Lint baseline stable. No new violations introduced.")

    sys.exit(0)

if __name__ == "__main__":
    main()

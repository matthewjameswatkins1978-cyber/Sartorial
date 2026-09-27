#!/usr/bin/env python3
"""
Language-Neutral Foreign Client Example: Python -> Sartorial

Demonstrates how any non-Rust language (Python, Node, Go, C#, OCaml, shell)
can use Sartorial as an external CLI presentation driver via JSON and JSONL.
"""

import json
import os
import subprocess
import sys
import time

def find_sartorial_binary():
    # Check local cargo build targets first, then PATH
    candidates = [
        os.path.join(os.path.dirname(__file__), "..", "target", "debug", "sartorial.exe"),
        os.path.join(os.path.dirname(__file__), "..", "target", "debug", "sartorial"),
        os.path.join(os.path.dirname(__file__), "..", "target", "release", "sartorial.exe"),
        os.path.join(os.path.dirname(__file__), "..", "target", "release", "sartorial"),
        "sartorial",
    ]
    for c in candidates:
        if os.path.exists(c):
            return os.path.abspath(c)
    return "sartorial"

def render_json(binary, payload, preset=None):
    cmd = [binary, "render"]
    if preset:
        cmd.extend(["--preset", preset])
    
    proc = subprocess.Popen(
        cmd,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    stdout, stderr = proc.communicate(input=json.dumps(payload))
    if stdout:
        sys.stdout.write(stdout)
    if stderr:
        sys.stderr.write(stderr)
    return proc.returncode

def stream_progress(binary, events):
    cmd = [binary, "stream"]
    proc = subprocess.Popen(
        cmd,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    
    for event in events:
        proc.stdin.write(json.dumps(event) + "\n")
        proc.stdin.flush()
        time.sleep(0.05) # simulate work
        
    proc.stdin.close()
    proc.wait()
    stdout, stderr = proc.communicate()
    if stdout:
        sys.stdout.write(stdout)
    if stderr:
        sys.stderr.write(stderr)
    return proc.returncode

def main():
    binary = find_sartorial_binary()
    print(f"--- Using Sartorial Driver: {binary} ---\n")

    # 1. Render Dry-Run Plan
    plan_payload = {
        "type": "plan",
        "title": "Migrate Cluster to Multi-Region",
        "description": "Orchestrated across us-east and eu-west regions",
        "changes": [
            {
                "kind": "add",
                "target": "eu-west-1 replica pool",
                "detail": "3 worker nodes"
            },
            {
                "kind": "modify",
                "target": "global DNS routing policy",
                "detail": "weighted latency failover"
            }
        ],
        "consequences": [
            "Network traffic will temporarily re-route during DNS propagation"
        ],
        "warnings": [
            "Verify IAM cross-region replication permissions before execution"
        ],
        "reversible": True,
        "actions": [
            {"id": "apply", "label": "Execute multi-region rollout", "trigger": {"char": "a"}}
        ]
    }
    render_json(binary, plan_payload, preset="studio")

    print("\n--- Simulating Live Operations via Streaming JSONL ---\n")

    # 2. Stream Live Progress
    progress_stream = [
        {"type": "progress.start", "id": "infra", "activity": "Provisioning eu-west-1 pool", "total": 3, "unit": "nodes"},
        {"type": "progress.update", "id": "infra", "current": 1, "subtask": "node-01 initialized"},
        {"type": "progress.update", "id": "infra", "current": 2, "subtask": "node-02 initialized"},
        {"type": "progress.update", "id": "infra", "current": 3, "subtask": "node-03 ready"},
        {"type": "progress.finish", "id": "infra", "status": "ready"},
    ]
    stream_progress(binary, progress_stream)

    print("\n--- Post-Operation Receipt ---\n")

    # 3. Render Post-Operation Receipt
    receipt_payload = {
        "type": "receipt",
        "title": "Cluster Migration Complete",
        "status": "ready",
        "changes": [
            {"name": "Region", "value": "us-east-1 + eu-west-1"},
            {"name": "Total Replicas", "value": "6 nodes"}
        ],
        "unchanged": [
            {"name": "Control Plane", "value": "k8s 1.30.2"}
        ],
        "guidance": "Traffic split active. Latency decreased by 42ms.",
        "warnings": [],
        "evidence_handle": "s3://audit/migration-2026-09-27.log",
        "actions": [
            {"id": "dashboard", "label": "Open CloudWatch metrics", "trigger": {"char": "d"}}
        ]
    }
    render_json(binary, receipt_payload, preset="house")

if __name__ == "__main__":
    sys.exit(main() or 0)

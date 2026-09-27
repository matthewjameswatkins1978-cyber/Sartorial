# Language-Neutral Foreign Client Example: PowerShell -> Sartorial
# Demonstrates Sartorial integration from PowerShell using JSON and JSONL streaming.

$ErrorActionPreference = "Stop"

function Find-SartorialBinary {
    $scriptDir = $PSScriptRoot
    $candidates = @(
        (Join-Path $scriptDir "..\target\debug\sartorial.exe"),
        (Join-Path $scriptDir "..\target\release\sartorial.exe"),
        "sartorial.exe",
        "sartorial"
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) {
            return (Resolve-Path $c).Path
        }
    }
    return "sartorial"
}

$binary = Find-SartorialBinary
Write-Host "--- Using Sartorial Driver: $binary ---`n"

# 1. Render Dry-Run Plan
$plan = @{
    type = "plan"
    title = "Deploy Service Mesh"
    description = "Rolling update for envoy sidecars"
    changes = @(
        @{ kind = "modify"; target = "ingress-gateway"; detail = "enable HTTP/2" },
        @{ kind = "add"; target = "telemetry-collector"; detail = "OTel daemonset" }
    )
    consequences = @("Transient connection resets during proxy rotation")
    warnings = @("Ensure cert-manager is healthy before running")
    reversible = $true
    actions = @(
        @{ id = "rollout"; label = "Execute rollout"; trigger = @{ char = "r" } }
    )
} | ConvertTo-Json -Depth 5

$plan | & $binary render --preset studio

Write-Host "`n--- Simulating Live Operations via Streaming JSONL ---`n"

# 2. Stream Live Progress
$events = @(
    '{"type":"progress.start","id":"mesh","activity":"Restarting proxies","total":3,"unit":"pods"}',
    '{"type":"progress.update","id":"mesh","current":1,"subtask":"pod-alpha restarted"}',
    '{"type":"progress.update","id":"mesh","current":2,"subtask":"pod-bravo restarted"}',
    '{"type":"progress.update","id":"mesh","current":3,"subtask":"pod-charlie restarted"}',
    '{"type":"progress.finish","id":"mesh","status":"ready"}'
)

$events | & $binary stream

Write-Host "`n--- Post-Operation Receipt ---`n"

# 3. Render Post-Operation Receipt
$receipt = @{
    type = "receipt"
    title = "Service Mesh Rollout"
    status = "ready"
    changes = @(
        @{ name = "Sidecars Updated"; value = "3 / 3 pods" },
        @{ name = "Protocol"; value = "HTTP/2 enabled" }
    )
    unchanged = @(
        @{ name = "Namespace"; value = "production" }
    )
    guidance = "Check telemetry stream for error rate metrics"
    warnings = @()
    evidence_handle = "logs://mesh-deploy-8812"
    actions = @(
        @{ id = "telemetry"; label = "View mesh telemetry"; trigger = @{ char = "t" } }
    )
} | ConvertTo-Json -Depth 5

$receipt | & $binary render --preset house

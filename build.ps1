cargo test --all-features
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo test failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}

cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo fmt failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}

cargo clippy --all --all-features --tests -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo clippy failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}
@echo off
cd /D %~dp0
cargo test -- --nocapture
pause
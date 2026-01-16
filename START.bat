@echo off
if exist comfy-qa.exe (
    comfy-qa.exe
) else if exist cli\target\release\comfy-qa.exe (
    cli\target\release\comfy-qa.exe
) else (
    echo ERROR: Cannot find comfy-qa.exe
    pause
)

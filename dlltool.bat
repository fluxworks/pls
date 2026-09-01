@echo off
where dlltool.exe > nul 2>&1
if %errorlevel% equ 0 (
    echo dlltool.exe found: Success!
) else (
    echo Error: dlltool.exe not found. Please check MinGW installation and PATH.
)
@echo off
where BCryptPrimitive.dll > nul 2>&1
if %errorlevel% equ 0 (
    echo BCryptPrimitive.dll found: Success!
) else (
    echo Error: BCryptPrimitive.dll not found. Please check MinGW installation and PATH.
)
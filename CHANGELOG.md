# Changelog

- December 7, 2025 at 3:45 AM

Add initial Tauri + Svelte application structure

This commit sets up the foundational structure for the Tauri application, including the addition of essential files such as package.json, vite.config.ts, and various Svelte components. It also includes a .gitignore file to exclude build artifacts and temporary files, and a README with basic usage instructions. The project is configured to utilize Tailwind CSS and includes initial UI components for a tabbed interface.

- December 7, 2025 at 3:48 AM

Remove lucide-svelte dependency from package.json and disable window maximization in tauri.conf.json

- December 7, 2025 at 5:16 AM

Update dependencies and enhance UI components

Downgraded @lucide/svelte from version 0.556.0 to 0.544.0 in package.json and package-lock.json.
Added new dependencies: mode-watcher (v1.1.0) and svelte-sonner (v1.0.7) for improved UI notifications.
Refactored App.svelte to integrate Toaster from svelte-sonner for better user feedback.
Enhanced StatsPanel and other components with improved layout and iconography for a more polished UI.
Introduced new dropdown menu components for better user interaction and organization.

- December 7, 2025 at 5:45 AM

Add package.json and package-lock.json for dependency management

Introduced package.json to manage project dependencies, including mode-watcher (v1.1.0).
Created package-lock.json to lock dependency versions and ensure consistent installations.
Updated App.svelte to integrate ModeWatcher for theme toggling and added a language selection dropdown for improved user experience.

- December 7, 2025 at 7:14 AM 

Add configuration files for Prettier and ESLint, and update package dependencies

Introduced .prettierrc and .prettierignore for consistent code formatting.
Added eslint.config.js for linting JavaScript and TypeScript code, including Svelte support.
Updated package.json and package-lock.json to include new dependencies for TypeScript ESLint.
Made minor adjustments to various Svelte components for improved styling and functionality.

- December 7, 2025 at 7:28 AM

Refactor code formatting in utils.ts and improve filename formatting in issues.rs

Adjusted indentation in the cn function for better readability in utils.ts.
Reformatted the filename string construction in export_issues_to_markdown for improved clarity in issues.rs.

- December 7, 2025 at 4:27 PM

Refactor project structure and enhance internationalization support

Renamed project from "repo-to-markdown-tauri" to "mergist" in package.json and related files.
Added support for internationalization using svelte-i18n, including locale detection and message translations for English, Russian, and Portuguese.
Updated various Svelte components to utilize translated strings for improved user experience.
Introduced new styles for better UI feedback and interaction, including disabling text selection globally while allowing it in input fields.
Enhanced the About section with dynamic app name and version retrieval, and added an Info icon for user access.

- December 7, 2025 at 4:46 PM

Remove the old MIT License and add Apache License 2.0 for the Tauri application. Update the README files to reflect the new project name "Mergist" and enhance documentation with features, requirements, and internationalization details.

// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const SHELL: string;
	export const VIM: string;
	export const MOTD_SHOWN: string;
	export const USER: string;
	export const XDG_MENU_PREFIX: string;
	export const SESSION_MANAGER: string;
	export const XDG_SEAT_PATH: string;
	export const OLDPWD: string;
	export const LANGUAGE: string;
	export const XDG_VTNR: string;
	export const WINDOWID: string;
	export const XDG_CONFIG_DIRS: string;
	export const PROFILEHOME: string;
	export const QT_WAYLAND_RECONNECT: string;
	export const XDG_SESSION_CLASS: string;
	export const QT_SCREEN_SCALE_FACTORS: string;
	export const SYSTEMD_EXEC_PID: string;
	export const KONSOLE_DBUS_WINDOW: string;
	export const GTK_RC_FILES: string;
	export const NVIM: string;
	export const JOURNAL_STREAM: string;
	export const MEMORY_PRESSURE_WRITE: string;
	export const QT_AUTO_SCREEN_SCALE_FACTOR: string;
	export const INVOCATION_ID: string;
	export const XDG_SESSION_DESKTOP: string;
	export const MANAGERPID: string;
	export const GTK2_RC_FILES: string;
	export const UBUNTU_MENUPROXY: string;
	export const PAM_KWALLET5_LOGIN: string;
	export const PWD: string;
	export const MAIL: string;
	export const KDE_APPLICATIONS_AS_SCOPE: string;
	export const APPDIR: string;
	export const VIMRUNTIME: string;
	export const XDG_SESSION_PATH: string;
	export const XDG_SEAT: string;
	export const GTK_MODULES: string;
	export const MEMORY_PRESSURE_WATCH: string;
	export const KONSOLE_VERSION: string;
	export const KONSOLE_DBUS_SESSION: string;
	export const OWD: string;
	export const LOGNAME: string;
	export const PATH: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const XDG_RUNTIME_DIR: string;
	export const KONSOLE_DBUS_SERVICE: string;
	export const XDG_SESSION_TYPE: string;
	export const COLORTERM: string;
	export const ICEAUTHORITY: string;
	export const APPIMAGE: string;
	export const LANG: string;
	export const KDE_SESSION_VERSION: string;
	export const XAUTHORITY: string;
	export const KDE_FULL_SESSION: string;
	export const XDG_CURRENT_DESKTOP: string;
	export const MYVIMRC: string;
	export const TERM: string;
	export const NVIM_LOG_FILE: string;
	export const DEBUGINFOD_URLS: string;
	export const SHELL_SESSION_ID: string;
	export const KDE_SESSION_UID: string;
	export const DESKTOP_SESSION: string;
	export const COLORFGBG: string;
	export const SHLVL: string;
	export const XDG_SESSION_ID: string;
	export const DISPLAY: string;
	export const HOME: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		SHELL: string;
		VIM: string;
		MOTD_SHOWN: string;
		USER: string;
		XDG_MENU_PREFIX: string;
		SESSION_MANAGER: string;
		XDG_SEAT_PATH: string;
		OLDPWD: string;
		LANGUAGE: string;
		XDG_VTNR: string;
		WINDOWID: string;
		XDG_CONFIG_DIRS: string;
		PROFILEHOME: string;
		QT_WAYLAND_RECONNECT: string;
		XDG_SESSION_CLASS: string;
		QT_SCREEN_SCALE_FACTORS: string;
		SYSTEMD_EXEC_PID: string;
		KONSOLE_DBUS_WINDOW: string;
		GTK_RC_FILES: string;
		NVIM: string;
		JOURNAL_STREAM: string;
		MEMORY_PRESSURE_WRITE: string;
		QT_AUTO_SCREEN_SCALE_FACTOR: string;
		INVOCATION_ID: string;
		XDG_SESSION_DESKTOP: string;
		MANAGERPID: string;
		GTK2_RC_FILES: string;
		UBUNTU_MENUPROXY: string;
		PAM_KWALLET5_LOGIN: string;
		PWD: string;
		MAIL: string;
		KDE_APPLICATIONS_AS_SCOPE: string;
		APPDIR: string;
		VIMRUNTIME: string;
		XDG_SESSION_PATH: string;
		XDG_SEAT: string;
		GTK_MODULES: string;
		MEMORY_PRESSURE_WATCH: string;
		KONSOLE_VERSION: string;
		KONSOLE_DBUS_SESSION: string;
		OWD: string;
		LOGNAME: string;
		PATH: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		XDG_RUNTIME_DIR: string;
		KONSOLE_DBUS_SERVICE: string;
		XDG_SESSION_TYPE: string;
		COLORTERM: string;
		ICEAUTHORITY: string;
		APPIMAGE: string;
		LANG: string;
		KDE_SESSION_VERSION: string;
		XAUTHORITY: string;
		KDE_FULL_SESSION: string;
		XDG_CURRENT_DESKTOP: string;
		MYVIMRC: string;
		TERM: string;
		NVIM_LOG_FILE: string;
		DEBUGINFOD_URLS: string;
		SHELL_SESSION_ID: string;
		KDE_SESSION_UID: string;
		DESKTOP_SESSION: string;
		COLORFGBG: string;
		SHLVL: string;
		XDG_SESSION_ID: string;
		DISPLAY: string;
		HOME: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}

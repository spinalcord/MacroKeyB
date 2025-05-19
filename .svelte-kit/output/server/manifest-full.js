export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["app.css","favicon.png","svelte.svg","tauri.svg","vite.svg"]),
	mimeTypes: {".css":"text/css",".png":"image/png",".svg":"image/svg+xml"},
	_: {
		client: {start:"_app/immutable/entry/start.DMefPf0r.js",app:"_app/immutable/entry/app.Di4fbMiy.js",imports:["_app/immutable/entry/start.DMefPf0r.js","_app/immutable/chunks/PlLXIb-l.js","_app/immutable/chunks/D6GfwYVl.js","_app/immutable/chunks/D_nVGoDb.js","_app/immutable/entry/app.Di4fbMiy.js","_app/immutable/chunks/D6GfwYVl.js","_app/immutable/chunks/Bt067wpS.js","_app/immutable/chunks/DD5P0bqb.js","_app/immutable/chunks/D9SFZBKc.js","_app/immutable/chunks/D_nVGoDb.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js'))
		],
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();

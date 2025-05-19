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
		client: {start:"_app/immutable/entry/start.BbW0sKnt.js",app:"_app/immutable/entry/app.BqMmfzu5.js",imports:["_app/immutable/entry/start.BbW0sKnt.js","_app/immutable/chunks/JSQIksqv.js","_app/immutable/chunks/CKyThxC_.js","_app/immutable/chunks/CkGDGDXt.js","_app/immutable/entry/app.BqMmfzu5.js","_app/immutable/chunks/CKyThxC_.js","_app/immutable/chunks/CwRoxpg3.js","_app/immutable/chunks/B7D040BA.js","_app/immutable/chunks/FyKIkh1t.js","_app/immutable/chunks/CkGDGDXt.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js'))
		],
		routes: [
			
		],
		prerendered_routes: new Set(["/"]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();

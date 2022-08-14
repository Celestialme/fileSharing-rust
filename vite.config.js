import { sveltekit } from '@sveltejs/kit/vite';
import Path from 'path';
/** @type {import('vite').UserConfig} */
const config = {
	plugins: [sveltekit()],
	server:{fs:{allow:['static']}},
		resolve:{
			alias:{
				'@src':Path.resolve('src/'),
				'@static':Path.resolve('static/')
			}
		},
};

export default config;

import type { PlaywrightTestConfig } from '@playwright/test';

const config: PlaywrightTestConfig = {
	webServer: {
		command: 'npm run build && npm run preview',
		port: 4173
	},
	testDir: './tests',
	reporter: [['list'], ['html']],
	use: {
		trace: 'on-first-retry'
	}
};

export default config;

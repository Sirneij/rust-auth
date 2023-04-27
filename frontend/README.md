# Frontend

## Run locally

Once you've cloned the application and installed it's dependencies, with `npm install` (or `pnpm install` or `yarn`), you need to locate `vite.config.ts` and uncomment line 3 and 6: `// import basicSsl from '@vitejs/plugin-basic-ssl';` and `// plugins: [basicSsl(), sveltekit()],`. Then comment line 7: `plugins: [sveltekit()],`. After that, start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

> **NOTE: You need to have node installed on your machine.**

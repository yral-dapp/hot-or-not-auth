import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'

export default defineConfig({
  plugins: [solid()],
  build: {
    sourcemap: true,
  },
  define: {
    // Here we can define global constants
    ...canisterDefinitions,
    'process.env.INTERNET_IDENTITY_CANISTER_ID': JSON.stringify(
      'qhbym-qaaaa-aaaaa-aaafq-cai',
    ),
    'process.env.CANISTER_ID_VERIFY_PRINCIPAL_BACKEND': JSON.stringify(
      'bkyz2-fmaaa-aaaaa-qaaaq-cai',
    ),
    'process.env.DFX_NETWORK': JSON.stringify(isDev ? 'local' : 'ic'),
    'import.meta.env.NODE_ENV': JSON.stringify(
      isDev ? 'development' : 'production',
    ),
    'import.meta.env.ENABLE_SSR': process.env.BUILD_MODE !== 'static',
    'import.meta.env.PRODUCTION': process.env.PRODUCTION === 'true',
  },
  optimizeDeps: {
    esbuildOptions: {
      // Node.js global to browser globalThis
      define: {
        global: 'globalThis',
      },
    },
    include: [
      '@dfinity/principal',
      '@dfinity/auth-client',
      '@dfinity/agent',
    ],
  },
})

const prodCanisterJson = async () =>
  await import('../verify_principal/canister_ids.json')

const devCanisterJson = async () => {
  try {
    return await import(
      //@ts-ignore
      '../verify_principal/.dfx/local/canister_ids.json'
    )
  } catch (e) {
    console.error(
      '⚠️ Error finding dev canister JSON. Did you forget to run `dfx deploy`?',
      e,
    )
    return {}
  }
}

const isDev = process.env.NODE_ENV !== 'production'

console.log('starting app in', isDev ? 'dev' : 'prod', 'mode')

const DFX_PORT = 4943

const canisterIds = isDev ? await devCanisterJson() : await prodCanisterJson()
// Generate canister ids, required by the generated canister code in .dfx/local/canisters/*
// This strange way of JSON.stringify the value is required by vite
const canisterDefinitions = Object.entries(canisterIds).reduce(
  (acc, [key, val]) => ({
    ...acc,
    [`process.env.CANISTER_ID_${key.toUpperCase()}`]: isDev
      ? JSON.stringify((val).local)
      : JSON.stringify((val).ic),
  }),
  {},
)

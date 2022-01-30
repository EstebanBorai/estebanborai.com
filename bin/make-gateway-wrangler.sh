#!/bin/bash


# read more about configuring your Worker via wrangler.toml at:
# https://developers.cloudflare.com/workers/cli-wrangler/configuration

# Environment Variables Required
# 
# - CF_ACCOUNT_ID: Account ID from Cloudflare
# - CF_ZONE_ID: Zone ID from Cloudflare
# - X_AUTH_TOKEN: Auth token for Gateway -> Server Authentication
# - API_URL: URL to Proxied Server Instance
echo -e "name = 'api-gateway'\ntype = 'javascript'\nworkers_dev = false\ncompatibility_date = '2022-01-30'\n\n[vars]\nWORKERS_RS_VERSION = '0.0.7'\nX_AUTH_TOKEN = '$X_AUTH_TOKEN'\nAPI_URL = '$API_URL'\n\n[build]\ncommand = 'cargo install -q worker-build && worker-build --release'\n\n[build.upload]\ndir    = 'build/worker'\nformat = 'modules'\nmain   = './shim.mjs'\n\n[[build.upload.rules]]\nglobs = ['**/*.wasm']\ntype  = 'CompiledWasm'\n\n[env.production]\nname = 'api'\nroute = 'api.estebanborai.com/*'\n\n[env.staging]\nname = 'api-staging'\nroute = 'api-staging.estebanborai.com/*'" > wrangler.toml

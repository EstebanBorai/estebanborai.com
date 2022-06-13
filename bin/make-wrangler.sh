#!/bin/bash

# Environment Variables Required
# 
# - CF_ACCOUNT_ID: Account ID from Cloudflare
# - CF_ZONE_ID: Zone ID from Cloudflare

WORKER_NAME="home"

echo -e "account_id = '$CF_ACCOUNT_ID'\ncompatibility_date = '2022-06-12'\nworkers_dev = false\nmain = './worker.mjs'\n\n[site]\nbucket = './.cloudflare/assets'\n[build]\ncommand = 'npm run build'\n\n[env.production]\nname = 'home'\nroute = 'estebanborai.com/*'\n\n[env.staging]\nname = 'home-staging'\nroute = 'staging.estebanborai.com/*'" > wrangler.toml

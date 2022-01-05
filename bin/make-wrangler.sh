#!/bin/bash

# Environment Variables Required
# 
# - CF_ACCOUNT_ID: Account ID from Cloudflare
# - CF_ZONE_ID: Zone ID from Cloudflare

WORKER_NAME="home"

echo -e "type = 'javascript'\ncompatibility_date = '2021-01-05'\naccount_id = '$CF_ACCOUNT_ID'\nworkers_dev = false\nzone_id = '$CF_ZONE_ID'\n\n[site]\nbucket = './build'\nentry-point = 'workers-site'\n\n[build]\ncommand = 'npm run build'\nwatch_dir = 'app'\n\n[build.upload]\nformat = 'service-worker'\n\n[env.production]\nname = 'home'\nroute = 'estebanborai.com/*'\n\n[env.staging]\nname = 'home-staging'\nroute = 'staging.estebanborai.com/*'" > wrangler.toml

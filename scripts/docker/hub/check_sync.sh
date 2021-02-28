#!/bin/bash
# checks if tetsy has a fully synced blockchain

VAP_SYNCING=$(curl -X POST --data '{"jsonrpc":"2.0","method":"vap_syncing","params":[],"id":1}' http://localhost:8545 -H 'Content-Type: application/json')
RESULT=$(echo "$VAP_SYNCING" | jq -r .result)

if [ "$RESULT" == "false" ]; then
  echo "Tetsy is ready to start accepting traffic"
  exit 0
else
  echo "Tetsy is still syncing the blockchain"
  exit 1
fi

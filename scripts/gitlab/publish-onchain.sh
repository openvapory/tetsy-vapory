#!/bin/bash

set -e # fail on any error
set -u # treat unset variables as error

echo "__________Register Release__________"
DATA="secret=$RELEASES_SECRET"

echo "Pushing release to Mainnet"
./tools/safe-curl.sh $DATA "https://update.tetcoin.org/push-release/${SCHEDULE_TAG:-${CI_COMMIT_REF_NAME}}/$CI_COMMIT_SHA"

cd artifacts
ls -l | sort -k9
filetest=( * )
echo ${filetest[*]}
for DIR in "${filetest[@]}";
do
  cd $DIR
  if [[ $DIR =~ "windows" ]];
    then
      WIN=".exe";
    else
      WIN="";
  fi
  sha3=$(cat tetsy.sha3 | awk '{print $1}')
  case $DIR in
    x86_64* )
      DATA="commit=$CI_COMMIT_SHA&sha3=$sha3&filename=tetsy$WIN&secret=$RELEASES_SECRET"
      ../../tools/safe-curl.sh $DATA "https://update.tetcoin.org/push-build/${SCHEDULE_TAG:-${CI_COMMIT_REF_NAME}}/$DIR"
      ;;
  esac
  cd ..
done

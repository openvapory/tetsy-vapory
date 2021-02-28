#!/usr/bin/env bash

cargo build --release -p vvmbin

./target/release/tetsy-vvm stats-jsontests-vm ./vapcore/res/vapory/tests/VMTests/vmArithmeticTest
./target/release/tetsy-vvm stats-jsontests-vm ./vapcore/res/vapory/tests/VMTests/vmBitwiseLogicOperation
./target/release/tetsy-vvm stats-jsontests-vm ./vapcore/res/vapory/tests/VMTests/vmBlockInfoTest
./target/release/tetsy-vvm stats-jsontests-vm ./vapcore/res/vapory/tests/VMTests/vmEnvironmentalInfo
./target/release/tetsy-vvm stats-jsontests-vm ./vapcore/res/vapory/tests/VMTests/vmIOandFlowOperations
./target/release/tetsy-vvm stats-jsontests-vm ./vapcore/res/vapory/tests/VMTests/vmLogTest
./target/release/tetsy-vvm stats-jsontests-vm ./vapcore/res/vapory/tests/VMTests/vmPerformance
./target/release/tetsy-vvm stats-jsontests-vm ./vapcore/res/vapory/tests/VMTests/vmPushDupSwapTest
./target/release/tetsy-vvm stats-jsontests-vm ./vapcore/res/vapory/tests/VMTests/vmRandomTest
./target/release/tetsy-vvm stats-jsontests-vm ./vapcore/res/vapory/tests/VMTests/vmSha3Test
./target/release/tetsy-vvm stats-jsontests-vm ./vapcore/res/vapory/tests/VMTests/vmSystemOperations
./target/release/tetsy-vvm stats-jsontests-vm ./vapcore/res/vapory/tests/VMTests/vmTests

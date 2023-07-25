## Description

ts-tests of tee-worker

## Environment

Node versions 16.x or 18.x, yarn v3

## Installation

`cd tee-worker/ts-tests`

`yarn`

## Type Generated

Update parachain metadata: `yarn workspace parachain-api update-metadata` (requires the parachain is running)

Update sidechain metadata: `yarn workspace sidechain-api update-metadata` (requires the worker is running)

Generate parachain type: `yarn workspace parachain-api build`

Generate sidechain type: `yarn workspace sidechain-api build`

## Local

[Start parachain && worker](https://github.com/litentry/litentry-parachain/blob/dev/README.md)

## Usage

Standard identity test: `yarn test-identity:local`

Standard vc test: `yarn test-vc:local`

Batch identity test: `yarn test-batch:local`

Bulk identity test: `yarn test-bulk-identity:local`

Bulk vc test: `yarn test-bulk-vc:local`

Direct invocation identity test: `yarn test-identity-direct-invocation:local`

Di examples: `yarn workspace integration-tests di-examples`
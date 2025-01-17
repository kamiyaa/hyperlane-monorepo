import { ethers } from 'ethers';

import {
  ChainMap,
  IsmConfig,
  OwnableConfig,
  RouterConfig,
  TokenRouterConfig,
  TokenType,
} from '@hyperlane-xyz/sdk';

import {
  RouterConfigWithoutOwner,
  tokens,
} from '../../../../../src/config/warp.js';

export const getBaseZeroNetworkCBBTCWarpConfig = async (
  routerConfig: ChainMap<RouterConfigWithoutOwner>,
  abacusWorksEnvOwnerConfig: ChainMap<OwnableConfig>,
): Promise<ChainMap<TokenRouterConfig>> => {
  const ISM_CONFIG: IsmConfig = ethers.constants.AddressZero;

  const base: TokenRouterConfig = {
    ...routerConfig.base,
    ...abacusWorksEnvOwnerConfig.base,
    proxyAdmin: {
      ...abacusWorksEnvOwnerConfig.base,
      address: '0x0FC41a92F526A8CD22060A4052e156502D6B9db0',
    },
    type: TokenType.collateral,
    token: tokens.base.cbBTC,
    interchainSecurityModule: ISM_CONFIG,
  };

  const zeronetwork: TokenRouterConfig = {
    ...routerConfig.zeronetwork,
    ...abacusWorksEnvOwnerConfig.zeronetwork,
    proxyAdmin: {
      ...abacusWorksEnvOwnerConfig.zeronetwork,
      address: '0xDb0F69187750b52A637938Ea790fAE667123367c',
    },
    type: TokenType.synthetic,
    interchainSecurityModule: ISM_CONFIG,
  };

  return {
    base,
    zeronetwork,
  };
};

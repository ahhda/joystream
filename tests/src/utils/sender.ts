import BN = require('bn.js');
import { ApiPromise } from '@polkadot/api';
import { Index } from '@polkadot/types/interfaces';
import { SubmittableExtrinsic } from '@polkadot/api/types';
import { KeyringPair } from '@polkadot/keyring/types';

export class Sender {
  private readonly api: ApiPromise;
  private nonceMap: Map<string, BN> = new Map();

  constructor(api: ApiPromise) {
    this.api = api;
  }

  private async getNonce(address: string): Promise<BN> {
    let nonce: BN | undefined = this.nonceMap.get(address);
    if (!nonce) {
      nonce = await this.api.query.system.accountNonce<Index>(address);
    }
    let nextNonce: BN = nonce.addn(1);
    this.nonceMap.set(address, nextNonce);
    return nonce;
  }

  private clearNonce(address: string): void {
    this.nonceMap.delete(address);
  }
}

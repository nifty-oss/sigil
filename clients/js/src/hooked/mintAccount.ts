import {
  Address,
  getAddressEncoder,
  getProgramDerivedAddress,
  getStringEncoder,
} from '@solana/web3.js';
import { TOKEN_LITE_PROGRAM_ADDRESS } from '../index.js';

export type MintAccountSeeds = {
  ticker: string;
  namespace: Address;
};

export const findMintAccountPda = async (seeds: MintAccountSeeds) => {
  // Seeds must be 32 bytes, so we take the four ticker bytes and the
  // first 28 bytes of the namespace address and concatenate them.
  const tickerBytes = getStringEncoder({ size: 4 }).encode(seeds.ticker);
  const namespaceBytes = Array.from(
    getAddressEncoder().encode(seeds.namespace).slice(0, 28)
  );

  return await getProgramDerivedAddress({
    seeds: [Uint8Array.from([...tickerBytes, ...namespaceBytes])],
    programAddress: TOKEN_LITE_PROGRAM_ADDRESS,
  });
};

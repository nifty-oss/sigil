/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Address,
  ProgramDerivedAddress,
  getAddressEncoder,
  getProgramDerivedAddress,
} from '@solana/addresses';
import { getStringEncoder } from '@solana/codecs';

export type TokenAccountSeeds = {
  /** The user of the token account */
  user: Address;
  /** The namespace of the token account */
  namespace: Address;
};

export async function findTokenAccountPda(
  seeds: TokenAccountSeeds,
  config: { programAddress?: Address | undefined } = {}
): Promise<ProgramDerivedAddress> {
  const {
    programAddress = 'BpPMgxYawb8Qiguavj3JccMdp7bTZWemSqJmDeYTsTD9' as Address<'BpPMgxYawb8Qiguavj3JccMdp7bTZWemSqJmDeYTsTD9'>,
  } = config;
  return await getProgramDerivedAddress({
    programAddress,
    seeds: [
      getStringEncoder({ size: 'variable' }).encode('token_account'),
      getAddressEncoder().encode(seeds.user),
      getAddressEncoder().encode(seeds.namespace),
    ],
  });
}

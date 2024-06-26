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

export type PocketSeeds = {
  /** The authority of the token account */
  authority: Address;
  /** The user of the token account */
  user: Address;
};

export async function findPocketPda(
  seeds: PocketSeeds,
  config: { programAddress?: Address | undefined } = {}
): Promise<ProgramDerivedAddress> {
  const {
    programAddress = 'BpPMgxYawb8Qiguavj3JccMdp7bTZWemSqJmDeYTsTD9' as Address<'BpPMgxYawb8Qiguavj3JccMdp7bTZWemSqJmDeYTsTD9'>,
  } = config;
  return await getProgramDerivedAddress({
    programAddress,
    seeds: [
      getStringEncoder({ size: 'variable' }).encode('pocket'),
      getAddressEncoder().encode(seeds.authority),
      getAddressEncoder().encode(seeds.user),
    ],
  });
}

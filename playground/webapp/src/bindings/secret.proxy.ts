/* This file is generated by zits. Do not edit manually */

import {REMOTE_ENDPOINT, DIRECT_SEND_TIMEOUT_MS, DIRECT_SEND_CHUNK_TIMEOUT_MS, DIRECTORY_PATH, DELIVERY_ZOME_NAME, DELIVERY_INTERGRITY_ZOME_NAME, DeliveryState, DistributionState, NoticeState, ParcelKind, DistributionStrategy, ItemKind, DeliveryNoticeQueryField, ReceptionProofQueryField, NoticeAckQueryField, SecretEntry, DeliverySummary, ParcelReference, ParcelDescription, Distribution, DeliveryNotice, NoticeAck, NoticeReply, ReplyAck, ParcelChunk, ParcelManifest, ReceptionProof, ReceptionAck, PendingItem, PublishParcelInput, DistributeParcelInput, RespondToNoticeInput, FetchChunkInput, GetNoticeOutput, CommitPendingItemInput, GetDeliveryStateInput, DeliveryProperties, SendSecretInput, Secret, } from './secret.types';
import {
/** types.ts */
HoloHash,
AgentPubKey,
DnaHash,
WasmHash,
EntryHash,
ActionHash,
AnyDhtHash,
ExternalHash,
KitsuneAgent,
KitsuneSpace,
HoloHashB64,
AgentPubKeyB64,
DnaHashB64,
WasmHashB64,
EntryHashB64,
ActionHashB64,
AnyDhtHashB64,
InstalledAppId,
Signature,
CellId,
DnaProperties,
RoleName,
InstalledCell,
Timestamp,
Duration,
HoloHashed,
NetworkInfo,
FetchPoolInfo,
/** hdk/action.ts */
SignedActionHashed,
ActionHashed,
ActionType,
Action,
NewEntryAction,
Dna,
AgentValidationPkg,
InitZomesComplete,
CreateLink,
DeleteLink,
OpenChain,
CloseChain,
Update,
Delete,
Create,
/** hdk/capabilities.ts */
CapSecret,
CapClaim,
GrantedFunctionsType,
GrantedFunctions,
ZomeCallCapGrant,
CapAccess,
CapGrant,
///** hdk/countersigning.ts */
//CounterSigningSessionData,
//PreflightRequest,
//CounterSigningSessionTimes,
//ActionBase,
//CounterSigningAgents,
//PreflightBytes,
//Role,
//CountersigningAgentState,
/** hdk/dht-ops.ts */
DhtOpType,
DhtOp,
getDhtOpType,
getDhtOpAction,
getDhtOpEntry,
getDhtOpSignature,
/** hdk/entry.ts */
EntryVisibility,
AppEntryDef,
EntryType,
EntryContent,
Entry,
/** hdk/record.ts */
Record as HcRecord,
RecordEntry as HcRecordEntry,
/** api/admin/types.ts */
InstalledAppInfoStatus,
DeactivationReason,
DisabledAppReason,
StemCell,
ProvisionedCell,
ClonedCell,
CellType,
CellInfo,
AppInfo,
MembraneProof,
FunctionName,
ZomeName,
ZomeDefinition,
IntegrityZome,
CoordinatorZome,
DnaDefinition,
ResourceBytes,
ResourceMap,
CellProvisioningStrategy,
CellProvisioning,
DnaVersionSpec,
DnaVersionFlexible,
AppRoleDnaManifest,
AppRoleManifest,
AppManifest,
AppBundle,
AppBundleSource,
NetworkSeed,
ZomeLocation,
   } from '@holochain/client';

import {
/** Common */
DhtOpHashB64,
DhtOpHash,
/** DnaFile */
DnaFile,
DnaDef,
Zomes,
WasmCode,
/** entry-details */
EntryDetails,
RecordDetails,
Details,
DetailsType,
EntryDhtStatus,
/** Validation */
ValidationStatus,
ValidationReceipt,
   } from '@holochain-open-dev/core-types';

/** User defined external dependencies */
export type EntryDefIndex = number;

import {ZomeProxy} from '@ddd-qc/lit-happ';
import {secretFunctionNames} from './secret.fn';

/**
 *
 */
export class SecretProxy extends ZomeProxy {
  static readonly DEFAULT_ZOME_NAME = "zSecret"
  static readonly FN_NAMES = secretFunctionNames
 

  async createSecret(value: string): Promise<EntryHash> {
    return this.call('create_secret', value);
  }

  async createSplitSecret(value: string): Promise<EntryHash> {
    return this.call('create_split_secret', value);
  }

  async getSecret(eh: EntryHash): Promise<string> {
    return this.call('get_secret', eh);
  }

  async getSecretsFrom(sender: AgentPubKey): Promise<EntryHash[]> {
    return this.call('get_secrets_from', sender);
  }

  async refuseSecret(parcelEh: EntryHash): Promise<EntryHash> {
    return this.call('refuse_secret', parcelEh);
  }

  async acceptSecret(parcelEh: EntryHash): Promise<EntryHash> {
    return this.call('accept_secret', parcelEh);
  }

  async sendSecret(input: SendSecretInput): Promise<ActionHash> {
    return this.call('send_secret', input);
  }
}
